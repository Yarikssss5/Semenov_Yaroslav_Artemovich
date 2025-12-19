package main

import (
	"context"
	"fmt"
	"log"
	"math"
	"math/rand"
	"os"
	"os/signal"
	"runtime"
	"strconv"
	"sync"
	"syscall"
	"time"

	"github.com/gofiber/fiber/v2"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/push"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/codes"
	"go.opentelemetry.io/otel/exporters/jaeger"
	"go.opentelemetry.io/otel/propagation"
	"go.opentelemetry.io/otel/sdk/resource"
	sdktrace "go.opentelemetry.io/otel/sdk/trace"
	semconv "go.opentelemetry.io/otel/semconv/v1.21.0"
	"go.opentelemetry.io/otel/trace"
)

// ==================== МОДЕЛИ ДАННЫХ ====================
type SensorConfig struct {
	AthleteID         string
	BaseHeartRate     int
	MaxHeartRate      int
	BaseFatigue       float64
	TrainingIntensity float64
	RecoveryRate      float64
	IsTraining        bool
	LastUpdate        time.Time
	TotalSteps        int64
	mu                sync.RWMutex
}

type SensorData struct {
	AthleteID        string       `json:"athlete_id"`
	DeviceID         string       `json:"device_id"`
	HeartRate        int          `json:"heart_rate"`
	HeartRateVar     float64      `json:"hr_variability"`
	BreathingRate    int          `json:"breathing_rate"`
	OxygenSaturation float64      `json:"oxygen_saturation"`
	Temperature      float64      `json:"temperature"`
	Steps            int          `json:"steps"`
	Speed            float64      `json:"speed"`
	Distance         float64      `json:"distance"`
	Calories         int          `json:"calories"`
	Fatigue          float64      `json:"fatigue"`
	HydrationLevel   float64      `json:"hydration"`
	MuscleLoad       []MuscleLoad `json:"muscle_load"`
	TotalSteps       int64        `json:"total_steps"`
	Timestamp        string       `json:"timestamp"`
}

type MuscleLoad struct {
	MuscleGroup string  `json:"muscle_group"`
	Load        float64 `json:"load"`
	Soreness    float64 `json:"soreness"`
}

type PhysiologicalData struct {
	HeartRate     int
	HRV           float64
	BreathingRate int
	Oxygen        float64
	Temperature   float64
	Hydration     float64
	Fatigue       float64
}

type ActivityData struct {
	Steps    int
	Speed    float64
	Distance float64
	Calories int
}

// ==================== ГЛОБАЛЬНЫЕ ПЕРЕМЕННЫЕ ====================
var (
	tracer          trace.Tracer
	metricsRegistry = prometheus.NewRegistry()

	// Глобальные счетчики
	globalSessionID = int64(1000)
	anomalyCounter  = int64(5)
	startTime       = time.Now()
	athleteConfigs  = sync.Map{}
	lastMetrics     = struct {
		sync.RWMutex
		values map[string]map[string]float64
	}{
		values: make(map[string]map[string]float64),
	}
	chaosEnabled      = true
	chaosLevel        = 100 // процент вероятности сбоя (0-100)
	chaosMutex        sync.RWMutex
	lastChaosEvent    time.Time
	corruptedAthletes = make(map[string]time.Time)
)

// ==================== PROMETHEUS МЕТРИКИ ====================
var (
	heartRateMetric = prometheus.NewGaugeVec(
		prometheus.GaugeOpts{
			Name: "athlete_heart_rate_bpm",
			Help: "Heart rate in BPM",
		},
		[]string{"athlete_id", "status"},
	)

	fatigueMetric = prometheus.NewGaugeVec(
		prometheus.GaugeOpts{
			Name: "athlete_fatigue_level",
			Help: "Fatigue level (0-1)",
		},
		[]string{"athlete_id"},
	)

	hydrationMetric = prometheus.NewGaugeVec(
		prometheus.GaugeOpts{
			Name: "athlete_hydration_level",
			Help: "Hydration level (0-1)",
		},
		[]string{"athlete_id"},
	)

	oxygenMetric = prometheus.NewGaugeVec(
		prometheus.GaugeOpts{
			Name: "athlete_oxygen_saturation",
			Help: "Oxygen saturation percentage",
		},
		[]string{"athlete_id"},
	)

	temperatureMetric = prometheus.NewGaugeVec(
		prometheus.GaugeOpts{
			Name: "athlete_temperature_celsius",
			Help: "Body temperature in Celsius",
		},
		[]string{"athlete_id"},
	)

	stepsMetric = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "athlete_steps_total",
			Help: "Total steps accumulated",
		},
		[]string{"athlete_id"},
	)

	caloriesMetric = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "athlete_calories_burned_total",
			Help: "Total calories burned",
		},
		[]string{"athlete_id"},
	)

	distanceMetric = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "athlete_distance_meters_total",
			Help: "Total distance in meters",
		},
		[]string{"athlete_id"},
	)

	sessionMetric = prometheus.NewCounter(
		prometheus.CounterOpts{
			Name: "athlete_sessions_total",
			Help: "Total training sessions",
		},
	)

	anomalyMetric = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "athlete_anomalies_detected_total",
			Help: "Total anomalies detected",
		},
		[]string{"type"},
	)

	httpRequestsMetric = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "http_api_requests_total",
			Help: "Total HTTP API requests",
		},
		[]string{"endpoint", "method", "status"},
	)

	heartRateHistogram = prometheus.NewHistogramVec(
		prometheus.HistogramOpts{
			Name:    "athlete_heart_rate_distribution",
			Help:    "Distribution of heart rate measurements",
			Buckets: prometheus.LinearBuckets(50, 10, 15),
		},
		[]string{"athlete_id"},
	)

	processingTimeHistogram = prometheus.NewHistogram(
		prometheus.HistogramOpts{
			Name:    "data_processing_duration_seconds",
			Help:    "Time taken to process sensor data",
			Buckets: []float64{0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0},
		},
	)
)

func clamp(value, min, max int) int {
	if value < min {
		return min
	}
	if value > max {
		return max
	}
	return value
}

// ==================== ИНИЦИАЛИЗАЦИЯ ====================
func init() {
	// Регистрируем метрики
	metricsRegistry.MustRegister(
		heartRateMetric,
		fatigueMetric,
		hydrationMetric,
		oxygenMetric,
		temperatureMetric,
		stepsMetric,
		caloriesMetric,
		distanceMetric,
		sessionMetric,
		anomalyMetric,
		httpRequestsMetric,
		heartRateHistogram,
		processingTimeHistogram,
	)

	// Инициализируем метрики начальными значениями
	initMetricsWithRealisticData()
}

func initTracer() (*sdktrace.TracerProvider, error) {
	exp, err := jaeger.New(jaeger.WithCollectorEndpoint(
		jaeger.WithEndpoint("http://localhost:14268/api/traces"),
	))
	if err != nil {
		return nil, err
	}
	tp := sdktrace.NewTracerProvider(
		sdktrace.WithBatcher(exp),
		sdktrace.WithSampler(sdktrace.AlwaysSample()),
		sdktrace.WithResource(resource.NewWithAttributes(
			semconv.SchemaURL,
			semconv.ServiceName("athlete-monitoring"),
			semconv.ServiceVersion("1.0.0"),
			attribute.String("environment", "development"),
		)),
	)
	otel.SetTracerProvider(tp)
	otel.SetTextMapPropagator(
		propagation.NewCompositeTextMapPropagator(
			propagation.TraceContext{},
			propagation.Baggage{},
		),
	)
	tracer = otel.Tracer("athlete-monitoring-service")
	return tp, nil
}

func tracingDebugMiddleware(c *fiber.Ctx) error {
	ctx := c.UserContext()

	// Проверяем, есть ли активный span
	span := trace.SpanFromContext(ctx)
	if !span.IsRecording() {
		log.Println("⚠️ No active span in context!")
	} else {
		log.Printf("✅ Active span: %s", span.SpanContext().TraceID().String())
	}

	return c.Next()
}

func initMetricsWithRealisticData() {
	log.Println("🎯 Инициализация метрик с реалистичными значениями...")

	initialData := map[string]struct {
		heartRate int
		fatigue   float64
		hydration float64
		oxygen    float64
		temp      float64
		steps     float64
		calories  float64
		distance  float64
		sessions  float64
		anomalies map[string]float64
	}{
		"athlete_001": {65, 0.25, 0.85, 97.5, 36.8, 12500, 420, 8500, 3,
			map[string]float64{"high_hr": 1, "low_hydration": 0}},
		"athlete_002": {72, 0.35, 0.78, 96.8, 37.1, 8900, 310, 6200, 2,
			map[string]float64{"high_temp": 2, "low_oxygen": 1}},
		"athlete_003": {80, 0.45, 0.72, 95.9, 37.3, 5400, 180, 3200, 1,
			map[string]float64{"high_fatigue": 3}},
		"athlete_004": {58, 0.15, 0.91, 98.2, 36.6, 18200, 610, 12400, 5,
			map[string]float64{"none": 0}},
		"athlete_005": {75, 0.30, 0.80, 97.0, 37.0, 11200, 390, 7800, 4,
			map[string]float64{"high_hr": 1, "high_temp": 1}},
	}

	for athleteID, data := range initialData {
		status := "resting"
		if rand.Float64() > 0.5 {
			status = "training"
		}

		heartRateMetric.WithLabelValues(athleteID, status).Set(float64(data.heartRate))
		fatigueMetric.WithLabelValues(athleteID).Set(data.fatigue)
		hydrationMetric.WithLabelValues(athleteID).Set(data.hydration)
		oxygenMetric.WithLabelValues(athleteID).Set(data.oxygen)
		temperatureMetric.WithLabelValues(athleteID).Set(data.temp)
		stepsMetric.WithLabelValues(athleteID).Add(data.steps)
		caloriesMetric.WithLabelValues(athleteID).Add(data.calories)
		distanceMetric.WithLabelValues(athleteID).Add(data.distance)

		for i := 0; i < 20; i++ {
			hr := float64(data.heartRate) + (rand.Float64()*20 - 10)
			heartRateHistogram.WithLabelValues(athleteID).Observe(hr)
		}

		for anomalyType, count := range data.anomalies {
			if count > 0 {
				anomalyMetric.WithLabelValues(anomalyType).Add(count)
			}
		}
	}
	sessionMetric.Add(15)
	globalSessionID = 1015
	httpRequestsMetric.WithLabelValues("/api/sensors", "GET", "200").Add(42)
	httpRequestsMetric.WithLabelValues("/api/stats", "GET", "200").Add(18)
	httpRequestsMetric.WithLabelValues("/health", "GET", "200").Add(56)
	for i := 0; i < 50; i++ {
		processingTimeHistogram.Observe(0.001 + rand.Float64()*0.01)
	}
	log.Println("✅ Метрики инициализированы")
}

func startChaosTask(ctx context.Context) {
	log.Println("🌀 Starting chaos background task...")

	// Тикер для случайных сбоев
	chaosTicker := time.NewTicker(20 * time.Second)
	defer chaosTicker.Stop()

	// Тикер для изменения уровня хаоса
	levelTicker := time.NewTicker(1 * time.Minute)
	defer levelTicker.Stop()

	for {
		select {
		case <-ctx.Done():
			log.Println("🌀 Chaos task stopped")
			return

		case <-chaosTicker.C:
			if chaosEnabled {
				triggerRandomChaos(ctx)
			}

		case <-levelTicker.C:
			// Случайно меняем уровень хаоса
			chaosMutex.Lock()
			change := rand.Intn(21) - 10 // -10 до +10
			chaosLevel += change
			if chaosLevel < 0 {
				chaosLevel = 0
			}
			if chaosLevel > 50 {
				chaosLevel = 50 // максимум 50%
			}
			chaosMutex.Unlock()
			log.Printf("🌀 Chaos level changed to %d%%", chaosLevel)
		}
	}
}

// Запустить случайный сбой
func triggerRandomChaos(ctx context.Context) {
	chaosMutex.RLock()
	level := chaosLevel
	chaosMutex.RUnlock()

	// Проверяем вероятность по уровню хаоса
	if rand.Intn(100) >= level {
		return
	}

	// Выбираем тип сбоя
	chaosTypes := []func(context.Context){
		spikeHeartRate,
		dropOxygen,
		overheatAthlete,
		dehydrateAthlete,
		crashSensor,
		simulateNetworkError,
	}

	chaosType := chaosTypes[rand.Intn(len(chaosTypes))]

	// Создаем span для отслеживания хаоса
	_, span := tracer.Start(ctx, "chaos-event",
		trace.WithAttributes(
			attribute.String("chaos.type", CallerName(0)),
			attribute.Int("chaos.level", level),
		))
	defer span.End()

	// Запускаем сбой
	chaosType(ctx)

	lastChaosEvent = time.Now()
	span.SetAttributes(
		attribute.String("chaos.time", lastChaosEvent.Format(time.RFC3339)),
	)

	log.Printf("🌀 Chaos triggered: %s (level: %d%%)", CallerName(0), level)
}

func CallerName(skip int) string {
	// skip + 1 accounts for CallerName itself
	pc, _, _, ok := runtime.Caller(skip + 1)
	if !ok {
		return ""
	}
	f := runtime.FuncForPC(pc)
	if f == nil {
		return ""
	}
	return f.Name() // Returns the fully-qualified name, e.g., "main.main" or "github.com"
}

// ==================== ФУНКЦИИ СБОЕВ ====================

// Резкий скачок пульса
func spikeHeartRate(ctx context.Context) {
	_, span := tracer.Start(ctx, "spike-heart-rate")
	defer span.End()

	// Выбираем случайного атлета
	athleteID := getRandomAthleteID()
	if athleteID == "" {
		return
	}

	// Портим данные в кэше
	if val, ok := athleteConfigs.Load(athleteID); ok {
		config := val.(*SensorConfig)
		config.mu.Lock()
		config.BaseHeartRate += 30 + rand.Intn(40)
		config.mu.Unlock()
	}

	// Создаем аномальную метрику
	heartRateMetric.WithLabelValues(athleteID, "critical").Set(float64(180 + rand.Intn(40)))
	anomalyMetric.WithLabelValues("chaos_heart_rate_spike").Inc()

	// Отмечаем атлета как испорченного
	corruptedAthletes[athleteID] = time.Now().Add(5 * time.Minute)

	span.SetAttributes(
		attribute.String("athlete.id", athleteID),
		attribute.String("effect", "heart_rate_spike"),
		attribute.Bool("is_critical", true),
	)
	span.SetStatus(codes.Error, "Chaos: Heart rate spike")
}

// Падение кислорода
func dropOxygen(ctx context.Context) {
	_, span := tracer.Start(ctx, "drop-oxygen")
	defer span.End()

	athleteID := getRandomAthleteID()
	if athleteID == "" {
		return
	}

	// Резко снижаем кислород
	oxygenValue := 85.0 + rand.Float64()*7.0 // 85-92%
	oxygenMetric.WithLabelValues(athleteID).Set(oxygenValue)
	anomalyMetric.WithLabelValues("chaos_oxygen_drop").Inc()

	span.SetAttributes(
		attribute.String("athlete.id", athleteID),
		attribute.Float64("oxygen_level", oxygenValue),
		attribute.Bool("is_critical", oxygenValue < 90),
	)

	if oxygenValue < 90 {
		span.SetStatus(codes.Error, "Chaos: Critical oxygen drop")
		span.RecordError(fmt.Errorf("oxygen dropped to %.1f%%", oxygenValue))
	}
}

// Перегрев атлета
func overheatAthlete(ctx context.Context) {
	_, span := tracer.Start(ctx, "overheat-athlete")
	defer span.End()

	athleteID := getRandomAthleteID()
	if athleteID == "" {
		return
	}

	// Высокая температура
	temp := 38.5 + rand.Float64()*2.0 // 38.5-40.5
	temperatureMetric.WithLabelValues(athleteID).Set(temp)
	anomalyMetric.WithLabelValues("chaos_high_temperature").Inc()

	span.SetAttributes(
		attribute.String("athlete.id", athleteID),
		attribute.Float64("temperature", temp),
	)

	if temp > 39.0 {
		span.SetStatus(codes.Error, "Chaos: High fever")
		span.RecordError(fmt.Errorf("temperature critical: %.1f°C", temp))
	}
}

// Обезвоживание
func dehydrateAthlete(ctx context.Context) {
	_, span := tracer.Start(ctx, "dehydrate-athlete")
	defer span.End()

	athleteID := getRandomAthleteID()
	if athleteID == "" {
		return
	}

	// Очень низкая гидратация
	hydration := 0.5 + rand.Float64()*0.15 // 0.5-0.65
	hydrationMetric.WithLabelValues(athleteID).Set(hydration)
	anomalyMetric.WithLabelValues("chaos_dehydration").Inc()

	// Портим конфиг
	if val, ok := athleteConfigs.Load(athleteID); ok {
		config := val.(*SensorConfig)
		config.mu.Lock()
		config.BaseFatigue += 0.3
		config.mu.Unlock()
	}

	span.SetAttributes(
		attribute.String("athlete.id", athleteID),
		attribute.Float64("hydration", hydration),
	)

	if hydration < 0.6 {
		span.SetStatus(codes.Error, "Chaos: Severe dehydration")
	}
}

// "Сломать" сенсор
func crashSensor(ctx context.Context) {
	_, span := tracer.Start(ctx, "crash-sensor")
	defer span.End()

	athleteID := getRandomAthleteID()
	if athleteID == "" {
		return
	}

	// Устанавливаем все метрики в ноль или некорректные значения
	heartRateMetric.WithLabelValues(athleteID, "error").Set(0)
	oxygenMetric.WithLabelValues(athleteID).Set(0)
	temperatureMetric.WithLabelValues(athleteID).Set(0)
	hydrationMetric.WithLabelValues(athleteID).Set(0)
	fatigueMetric.WithLabelValues(athleteID).Set(1.0)

	anomalyMetric.WithLabelValues("chaos_sensor_crash").Inc()

	span.SetAttributes(
		attribute.String("athlete.id", athleteID),
		attribute.String("effect", "sensor_crash"),
	)
	span.SetStatus(codes.Error, "Chaos: Sensor crash")
	span.RecordError(fmt.Errorf("sensor %s crashed", athleteID))
}

// Имитация сетевой ошибки
func simulateNetworkError(ctx context.Context) {
	_, span := tracer.Start(ctx, "network-error")
	defer span.End()

	// Увеличиваем счетчик ошибок HTTP
	httpRequestsMetric.WithLabelValues("/api/sensors", "GET", "500").Inc()
	httpRequestsMetric.WithLabelValues("/api/stats", "GET", "503").Inc()

	// Создаем фейковую ошибку в метриках обработки
	for i := 0; i < 3; i++ {
		processingTimeHistogram.Observe(2.0 + rand.Float64()*3.0) // Очень долгая обработка
	}

	span.SetAttributes(
		attribute.String("error.type", "network"),
		attribute.Int("simulated_errors", 3),
	)
	span.SetStatus(codes.Error, "Chaos: Network error simulated")
}

// ==================== УТИЛИТЫ ====================

// Получить случайный ID атлета
func getRandomAthleteID() string {
	athletes := []string{
		"athlete_001",
		"athlete_002",
		"athlete_003",
		"athlete_004",
		"athlete_005",
	}
	return athletes[rand.Intn(len(athletes))]
}

// API для управления хаосом
func chaosControlHandler(c *fiber.Ctx) error {
	ctx := c.UserContext()

	var request struct {
		Enabled *bool `json:"enabled"`
		Level   *int  `json:"level"`
	}

	if err := c.BodyParser(&request); err != nil {
		return c.Status(400).JSON(fiber.Map{"error": "Invalid request"})
	}

	_, span := tracer.Start(ctx, "chaos-control")
	defer span.End()

	chaosMutex.Lock()

	if request.Enabled != nil {
		chaosEnabled = *request.Enabled
		span.SetAttributes(attribute.Bool("chaos.enabled", chaosEnabled))
	}

	if request.Level != nil {
		level := *request.Level
		if level >= 0 && level <= 100 {
			chaosLevel = level
		}
		span.SetAttributes(attribute.Int("chaos.level", chaosLevel))
	}

	currentLevel := chaosLevel
	currentEnabled := chaosEnabled
	chaosMutex.Unlock()

	span.SetStatus(codes.Ok, "Chaos control updated")

	return c.JSON(fiber.Map{
		"chaos_enabled": currentEnabled,
		"chaos_level":   currentLevel,
		"message":       "Chaos settings updated",
		"last_event":    lastChaosEvent.Format(time.RFC3339),
	})
}

type responseWriter struct {
	statusCode int
}

func (rw *responseWriter) WriteHeader(code int) {
	rw.statusCode = code
}

func tracingMiddleware(c *fiber.Ctx) error {
	spanName := fmt.Sprintf("%s %s", c.Method(), c.Path())
	ctx, span := tracer.Start(c.Context(), spanName,
		trace.WithSpanKind(trace.SpanKindServer),
		trace.WithAttributes(
			attribute.String("http.method", c.Method()),
			attribute.String("http.path", c.Path()),
			attribute.String("http.user_agent", c.Get("User-Agent")),
			attribute.String("http.client_ip", c.IP()),
		),
	)
	defer span.End()
	c.SetUserContext(ctx)
	err := c.Next()
	statusCode := c.Response().StatusCode()
	span.SetAttributes(
		attribute.Int("http.status_code", statusCode),
		attribute.Bool("http.success", statusCode < 400),
	)
	if statusCode >= 400 {
		span.SetStatus(codes.Error, fmt.Sprintf("HTTP %d", statusCode))
		span.RecordError(fmt.Errorf("request failed with status %d", statusCode))
	}
	httpRequestsMetric.WithLabelValues(c.Path(), c.Method(), strconv.Itoa(statusCode)).Inc()
	return err
}

func generateSensorDataWithTracing(ctx context.Context, athleteID string) SensorData {
	ctx, span := tracer.Start(ctx, "generate-sensor-data",
		trace.WithAttributes(attribute.String("athlete.id", athleteID)))
	defer span.End()

	startTime := time.Now()

	config := getAthleteConfigWithTracing(ctx, athleteID)
	physioData := generatePhysiologicalDataWithTracing(ctx, config)
	activityData := generateActivityDataWithTracing(ctx, config)
	muscleLoad := generateMuscleLoadWithTracing(ctx, config, physioData.Fatigue)
	detectAnomaliesWithTracing(ctx, athleteID, physioData)

	data := SensorData{
		AthleteID:        athleteID,
		DeviceID:         fmt.Sprintf("fitbit_%s", athleteID[len(athleteID)-3:]),
		HeartRate:        physioData.HeartRate,
		HeartRateVar:     physioData.HRV,
		BreathingRate:    physioData.BreathingRate,
		OxygenSaturation: physioData.Oxygen,
		Temperature:      physioData.Temperature,
		HydrationLevel:   physioData.Hydration,
		Steps:            activityData.Steps,
		Speed:            activityData.Speed,
		Distance:         activityData.Distance,
		Calories:         activityData.Calories,
		Fatigue:          physioData.Fatigue,
		MuscleLoad:       muscleLoad,
		TotalSteps:       config.TotalSteps,
		Timestamp:        time.Now().Format(time.RFC3339Nano),
	}

	config.TotalSteps += int64(data.Steps)
	config.LastUpdate = time.Now()

	duration := time.Since(startTime)
	span.SetAttributes(
		attribute.Int("heart_rate", data.HeartRate),
		attribute.Float64("processing_duration_ms", float64(duration.Milliseconds())),
	)

	updateMetricsWithTracing(ctx, data)
	processingTimeHistogram.Observe(duration.Seconds())

	return data
}

func getAthleteConfigWithTracing(ctx context.Context, athleteID string) *SensorConfig {
	_, span := tracer.Start(ctx, "get-athlete-config",
		trace.WithAttributes(attribute.String("athlete.id", athleteID)))
	defer span.End()

	// Пытаемся получить конфигурацию из хранилища
	var config *SensorConfig
	exists := false

	if val, ok := athleteConfigs.Load(athleteID); ok {
		config = val.(*SensorConfig)
		exists = true
		span.AddEvent("config.found_in_cache")
	}

	// Если конфигурации нет, создаем новую
	if !exists {
		config = &SensorConfig{
			AthleteID:         athleteID,
			BaseHeartRate:     65 + rand.Intn(20),
			MaxHeartRate:      200,
			BaseFatigue:       0.2 + rand.Float64()*0.3,
			TrainingIntensity: 0.4 + rand.Float64()*0.3,
			RecoveryRate:      0.15 + rand.Float64()*0.1,
			IsTraining:        rand.Float64() > 0.4,
			TotalSteps:        int64(5000 + rand.Intn(20000)),
			LastUpdate:        time.Now().Add(-time.Duration(rand.Intn(30)) * time.Minute),
		}

		// Сохраняем новую конфигурацию
		athleteConfigs.Store(athleteID, config)
		span.AddEvent("athlete.created")
	}

	span.SetAttributes(
		attribute.Bool("is_training", config.IsTraining),
		attribute.Int64("total_steps", config.TotalSteps),
		attribute.String("last_update", config.LastUpdate.Format(time.RFC3339)),
	)

	return config
}

func generatePhysiologicalDataWithTracing(ctx context.Context, config *SensorConfig) PhysiologicalData {
	ctx, span := tracer.Start(ctx, "generate-physiological-data")
	defer span.End()
	time.Sleep(time.Millisecond * time.Duration(10+rand.Intn(20)))
	data := PhysiologicalData{
		HeartRate:     generateHeartRate(config, 0),
		HRV:           generateHRV(config, config.IsTraining),
		BreathingRate: generateBreathingRate(config, config.IsTraining),
		Oxygen:        generateOxygenSaturation(config.BaseHeartRate),
		Temperature:   generateTemperature(config, config.IsTraining),
		Hydration:     generateHydrationLevel(config, 30),
		Fatigue:       generateFatigue(config, config.IsTraining, 30),
	}
	span.SetAttributes(
		attribute.Int("heart_rate", data.HeartRate),
		attribute.Float64("oxygen", data.Oxygen),
		attribute.Float64("fatigue", data.Fatigue),
	)
	return data
}

// Новые handler-ы для цепочки вызовов
func analyzeDataWithTracing(ctx context.Context, sensorData SensorData) fiber.Map {
	_, span := tracer.Start(ctx, "analyze-sensor-data",
		trace.WithAttributes(
			attribute.String("athlete.id", sensorData.AthleteID),
			attribute.Int("heart_rate", sensorData.HeartRate),
		))
	defer span.End()

	// Анализ данных
	healthScore := 100.0

	// Проверка пульса (норма: 60-100)
	if sensorData.HeartRate < 60 {
		healthScore -= 20
	} else if sensorData.HeartRate > 100 {
		healthScore -= 15
	} else if sensorData.HeartRate > 140 {
		healthScore -= 30
	}

	// Проверка кислорода (норма: >95%)
	if sensorData.OxygenSaturation < 95 {
		healthScore -= 25
	} else if sensorData.OxygenSaturation < 92 {
		healthScore -= 40
	}

	// Проверка гидратации (норма: >0.7)
	if sensorData.HydrationLevel < 0.7 {
		healthScore -= 20
	}

	// Проверка усталости
	if sensorData.Fatigue > 0.8 {
		healthScore -= 30
	} else if sensorData.Fatigue > 0.6 {
		healthScore -= 15
	}

	// Определение статуса
	status := "excellent"
	if healthScore >= 90 {
		status = "excellent"
	} else if healthScore >= 70 {
		status = "good"
	} else if healthScore >= 50 {
		status = "moderate"
	} else {
		status = "poor"
	}

	// Рекомендации
	var recommendations []string
	if sensorData.HeartRate > 100 {
		recommendations = append(recommendations, "consider resting, heart rate is elevated")
	}
	if sensorData.OxygenSaturation < 95 {
		recommendations = append(recommendations, "check breathing, oxygen level is low")
	}
	if sensorData.HydrationLevel < 0.7 {
		recommendations = append(recommendations, "drink water, hydration is low")
	}
	if sensorData.Fatigue > 0.6 {
		recommendations = append(recommendations, "consider rest or recovery")
	}
	if len(recommendations) == 0 {
		recommendations = append(recommendations, "all parameters are within normal range")
	}

	result := fiber.Map{
		"health_score": math.Round(healthScore*10) / 10,
		"status":       status,
		"athlete_id":   sensorData.AthleteID,
		"timestamp":    sensorData.Timestamp,
		"key_metrics": fiber.Map{
			"heart_rate":  sensorData.HeartRate,
			"oxygen":      sensorData.OxygenSaturation,
			"hydration":   sensorData.HydrationLevel,
			"fatigue":     sensorData.Fatigue,
			"temperature": sensorData.Temperature,
		},
		"recommendations": recommendations,
		"analysis_time":   time.Now().Format(time.RFC3339),
	}

	span.SetAttributes(
		attribute.Float64("health_score", healthScore),
		attribute.String("health_status", status),
		attribute.Int("recommendations_count", len(recommendations)),
	)

	return result
}

func generateActivityDataWithTracing(ctx context.Context, config *SensorConfig) ActivityData {
	ctx, span := tracer.Start(ctx, "generate-activity-data")
	defer span.End()
	steps := generateSteps(config, 30)
	distance := float64(steps) * float64(0.0008)
	data := ActivityData{
		Steps:    steps,
		Speed:    generateSpeed(config),
		Distance: distance,
		Calories: int(float64(steps) * 0.04),
	}
	span.SetAttributes(
		attribute.Int("steps", data.Steps),
		attribute.Float64("speed", data.Speed),
		attribute.Int("calories", data.Calories),
	)
	return data
}

func generateMuscleLoadWithTracing(ctx context.Context, config *SensorConfig, fatigue float64) []MuscleLoad {
	ctx, span := tracer.Start(ctx, "generate-muscle-load")
	defer span.End()
	groups := []string{"quadriceps", "hamstrings", "calves", "glutes", "core"}
	var loads []MuscleLoad
	for _, group := range groups {
		baseLoad := 0.15 + rand.Float64()*0.3
		if config.IsTraining {
			baseLoad += 0.2
		}
		soreness := baseLoad*0.7 + fatigue*0.2
		loads = append(loads, MuscleLoad{
			MuscleGroup: group,
			Load:        clampFloat(baseLoad, 0.1, 0.95),
			Soreness:    clampFloat(soreness, 0.1, 0.9),
		})
	}
	span.SetAttributes(attribute.Int("muscle_groups", len(loads)))
	return loads
}

func detectAnomaliesWithTracing(ctx context.Context, athleteID string, data PhysiologicalData) {
	ctx, span := tracer.Start(ctx, "detect-anomalies",
		trace.WithAttributes(attribute.String("athlete.id", athleteID)))
	defer span.End()
	anomalies := []string{}
	if data.HeartRate > 180 {
		anomalies = append(anomalies, "high_heart_rate")
		anomalyMetric.WithLabelValues("high_heart_rate").Inc()
		span.SetStatus(codes.Error, "Critical high heart rate detected")
		span.AddEvent("anomaly.detected", trace.WithAttributes(
			attribute.String("type", "high_heart_rate"),
			attribute.Int("value", data.HeartRate),
		))
	}
	if data.Oxygen < 92 {
		anomalies = append(anomalies, "low_oxygen")
		anomalyMetric.WithLabelValues("low_oxygen").Inc()
		span.SetStatus(codes.Error, "Critical low oxygen saturation")
		span.AddEvent("anomaly.detected", trace.WithAttributes(
			attribute.String("type", "low_oxygen"),
			attribute.Float64("value", data.Oxygen),
		))
	}
	if data.Hydration < 0.65 {
		anomalies = append(anomalies, "dehydration")
		anomalyMetric.WithLabelValues("dehydration").Inc()
		span.AddEvent("anomaly.detected", trace.WithAttributes(
			attribute.String("type", "dehydration"),
			attribute.Float64("value", data.Hydration),
		))
	}
	span.SetAttributes(
		attribute.StringSlice("detected_anomalies", anomalies),
		attribute.Int("anomalies_count", len(anomalies)),
	)
}

// ==================== ГЕНЕРАТОРЫ ФИЗИОЛОГИЧЕСКИХ ДАННЫХ ====================
func generateHeartRate(config *SensorConfig, timeSinceUpdate float64) int {
	baseHR := config.BaseHeartRate
	maxHR := config.MaxHeartRate
	if config.IsTraining {
		intensity := config.TrainingIntensity
		targetHR := baseHR + int(float64(maxHR-baseHR)*intensity)
		timeFactor := math.Min(1.0, timeSinceUpdate/300.0)
		currentHR := baseHR + int(float64(targetHR-baseHR)*timeFactor)
		sinWave := int(8 * math.Sin(timeSinceUpdate*0.3))
		randomNoise := rand.Intn(7) - 3
		return clamp(currentHR+sinWave+randomNoise, baseHR+20, maxHR-5)
	}
	hour := float64(time.Now().Hour())
	dailyFactor := 0.8 + 0.2*math.Sin((hour-14)*math.Pi/12)
	restingHR := float64(baseHR) * dailyFactor
	return clamp(int(restingHR)+rand.Intn(5)-2, 40, baseHR+15)
}

func generateHRV(config *SensorConfig, isTraining bool) float64 {
	baseHRV := 60.0 - float64(config.BaseHeartRate-50)
	if isTraining {
		return baseHRV * (0.6 + rand.Float64()*0.3)
	}
	return baseHRV * (0.9 + rand.Float64()*0.4)
}

func generateOxygenSaturation(heartRate int) float64 {
	base := 97.0 + rand.Float64()*2.0
	if heartRate > 160 {
		reduction := float64(heartRate-160) * 0.02
		return math.Max(92.0, base-reduction)
	}
	return base
}

func generateTemperature(config *SensorConfig, isTraining bool) float64 {
	base := 36.6 + rand.Float64()*0.5
	if isTraining {
		increase := config.TrainingIntensity * 0.8
		return math.Min(38.0, base+increase)
	}
	return base
}

func generateHydrationLevel(config *SensorConfig, timeSinceUpdate float64) float64 {
	base := 0.78 + rand.Float64()*0.15
	dehydration := timeSinceUpdate * 0.0003
	if config.IsTraining {
		dehydration *= 2.5
	}
	return math.Max(0.65, base-dehydration)
}

func generateFatigue(config *SensorConfig, isTraining bool, timeSinceUpdate float64) float64 {
	if isTraining {
		accumulation := timeSinceUpdate * 0.0015 * config.TrainingIntensity
		fatigue := config.BaseFatigue + accumulation
		fatigue += (rand.Float64() - 0.5) * 0.08
		return clampFloat(fatigue, 0.1, 0.95)
	}
	recovery := timeSinceUpdate * 0.0008 * config.RecoveryRate
	fatigue := config.BaseFatigue - recovery
	return math.Max(0.05, fatigue)
}

func clampFloat(value, min, max float64) float64 {
	if value < min {
		return min
	}
	if value > max {
		return max
	}
	return value
}

func generateSteps(config *SensorConfig, timeSinceUpdate float64) int {
	if !config.IsTraining {
		return rand.Intn(20)
	}

	stepsPerMinute := 0
	switch {
	case config.TrainingIntensity < 0.4:
		stepsPerMinute = 80 + rand.Intn(40)
	case config.TrainingIntensity < 0.7:
		stepsPerMinute = 120 + rand.Intn(60)
	default:
		stepsPerMinute = 180 + rand.Intn(80)
	}

	return int(float64(stepsPerMinute) * (timeSinceUpdate / 60))
}

func generateSpeed(config *SensorConfig) float64 {
	if !config.IsTraining {
		return 0
	}

	switch config.AthleteID {
	case "athlete_001":
		return 12.5 + rand.Float64()*3.5
	case "athlete_002":
		return 25.0 + rand.Float64()*8.0
	case "athlete_004":
		return 3.5 + rand.Float64()*2.0
	default:
		return 7.0 + rand.Float64()*5.0
	}
}

func generateBreathingRate(config *SensorConfig, isTraining bool) int {
	if isTraining {
		return 16 + rand.Intn(12) + int(config.TrainingIntensity*10)
	}
	return 12 + rand.Intn(6)
}

// ==================== METRICS MANAGEMENT ====================

func pushMetricsWithTracing(ctx context.Context) error {
	ctx, span := tracer.Start(ctx, "push-metrics")
	defer span.End()

	pusher := push.New("http://localhost:9091", "athlete_monitoring").
		Gatherer(metricsRegistry).
		Grouping("service", "sensor-api").
		Grouping("environment", "production")

	timestampMetric := prometheus.NewGauge(prometheus.GaugeOpts{
		Name: "metrics_push_timestamp",
		Help: "Last metrics push time",
	})
	timestampMetric.SetToCurrentTime()
	pusher.Collector(timestampMetric)

	ctx, cancel := context.WithTimeout(ctx, 5*time.Second)
	defer cancel()

	start := time.Now()
	err := pusher.PushContext(ctx)
	duration := time.Since(start)

	span.SetAttributes(
		attribute.Float64("push_duration_ms", float64(duration.Milliseconds())),
		attribute.Bool("push_successful", err == nil),
	)

	if err != nil {
		span.RecordError(err)
		span.SetStatus(codes.Error, "Push failed")
		log.Printf("❌ Push failed: %v", err)
		return err
	}

	span.AddEvent("metrics.pushed")
	log.Printf("✅ Metrics pushed at %s", time.Now().Format("15:04:05"))
	return nil
}

// ==================== HTTP HANDLERS ====================

// ==================== HTTP HANDLERS ====================
func getSensorDataHandler(c *fiber.Ctx) error {
	ctx := c.UserContext()
	athleteID := c.Params("id")
	if athleteID == "" {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "athlete_id is required",
		})
	}

	data := generateSensorDataWithTracing(ctx, athleteID)
	return c.JSON(data)
}

func getMultipleSensorDataHandler(c *fiber.Ctx) error {
	ctx := c.UserContext()

	var request struct {
		AthleteIDs []string `json:"athlete_ids"`
		Limit      int      `json:"limit" default:"10"`
	}

	if err := c.BodyParser(&request); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "invalid request body",
		})
	}

	if len(request.AthleteIDs) == 0 {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "athlete_ids array is required",
		})
	}

	if request.Limit <= 0 || request.Limit > 50 {
		request.Limit = 10
	}

	_, span := tracer.Start(ctx, "get-multiple-sensor-data",
		trace.WithAttributes(
			attribute.Int("athletes_count", len(request.AthleteIDs)),
			attribute.Int("limit", request.Limit),
		))
	defer span.End()

	var results []SensorData
	for i, athleteID := range request.AthleteIDs {
		if i >= request.Limit {
			break
		}
		data := generateSensorDataWithTracing(ctx, athleteID)
		results = append(results, data)
	}

	span.SetAttributes(attribute.Int("results_count", len(results)))
	return c.JSON(fiber.Map{
		"results": results,
		"count":   len(results),
	})
}

func getAthleteStatsHandler(c *fiber.Ctx) error {
	ctx := c.UserContext()
	athleteID := c.Params("id")
	if athleteID == "" {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "athlete_id is required",
		})
	}

	ctx, span := tracer.Start(ctx, "get-athlete-stats",
		trace.WithAttributes(attribute.String("athlete.id", athleteID)))
	defer span.End()

	config := getAthleteConfigWithTracing(ctx, athleteID)

	// Собираем текущие значения метрик
	stats := fiber.Map{
		"athlete_id":          athleteID,
		"is_training":         config.IsTraining,
		"base_heart_rate":     config.BaseHeartRate,
		"max_heart_rate":      config.MaxHeartRate,
		"base_fatigue":        config.BaseFatigue,
		"training_intensity":  config.TrainingIntensity,
		"recovery_rate":       config.RecoveryRate,
		"total_steps":         config.TotalSteps,
		"last_update":         config.LastUpdate.Format(time.RFC3339),
		"time_since_update":   time.Since(config.LastUpdate).String(),
		"current_heart_rate":  getCurrentHeartRateMetric(athleteID),
		"current_fatigue":     getCurrentFatigueMetric(athleteID),
		"current_hydration":   getCurrentHydrationMetric(athleteID),
		"current_oxygen":      getCurrentOxygenMetric(athleteID),
		"current_temperature": getCurrentTemperatureMetric(athleteID),
		"total_calories":      getTotalCaloriesMetric(athleteID),
		"total_distance":      getTotalDistanceMetric(athleteID),
		"anomalies_count":     getAnomaliesCount(athleteID),
	}

	return c.JSON(stats)
}

func getMetricsHandler(c *fiber.Ctx) error {
	ctx := c.UserContext()

	_, span := tracer.Start(ctx, "get-metrics")
	defer span.End()

	metrics, err := metricsRegistry.Gather()
	if err != nil {
		span.RecordError(err)
		span.SetStatus(codes.Error, "failed to gather metrics")
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "failed to gather metrics",
		})
	}

	var result []fiber.Map
	for _, metric := range metrics {
		metricData := fiber.Map{
			"name":   metric.GetName(),
			"help":   metric.GetHelp(),
			"type":   metric.GetType().String(),
			"metric": metric,
		}
		result = append(result, metricData)
	}

	span.SetAttributes(attribute.Int("metrics_count", len(result)))
	return c.JSON(fiber.Map{
		"metrics": result,
		"count":   len(result),
	})
}

func pushMetricsHandler(c *fiber.Ctx) error {
	ctx := c.UserContext()

	_, span := tracer.Start(ctx, "push-metrics-handler")
	defer span.End()

	err := pushMetricsWithTracing(ctx)
	if err != nil {
		span.RecordError(err)
		span.SetStatus(codes.Error, "push failed")
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error":   "failed to push metrics",
			"details": err.Error(),
		})
	}

	return c.JSON(fiber.Map{
		"success": true,
		"message": "metrics pushed successfully",
		"time":    time.Now().Format(time.RFC3339),
	})
}

// Новый health handler с цепочкой вызовов
func healthCheckHandler(c *fiber.Ctx) error {
	ctx := c.UserContext()

	_, span := tracer.Start(ctx, "health-check-chain")
	defer span.End()

	// 1. Получаем данные с сенсора (используем athlete_001 для демо)
	sensorData := generateSensorDataWithTracing(ctx, "athlete_001")
	span.AddEvent("sensor.data.generated")

	// 2. Анализируем данные
	analysisResult := analyzeDataWithTracing(ctx, sensorData)
	span.AddEvent("data.analyzed")

	// 3. Формируем полный ответ
	response := fiber.Map{
		"status":    "healthy",
		"timestamp": time.Now().Format(time.RFC3339),
		"uptime":    time.Since(startTime).String(),
		"version":   "1.0.0",
		"chain": []string{
			"sensor → analyze → health",
		},
		"analysis": analysisResult,
		"services": fiber.Map{
			"sensor":  "operational",
			"analyze": "operational",
			"api":     "operational",
			"tracing": "operational",
			"metrics": "operational",
		},
	}

	span.SetAttributes(
		attribute.String("health_status", "healthy"),
		attribute.Float64("health_score", analysisResult["health_score"].(float64)),
		attribute.Float64("uptime_seconds", time.Since(startTime).Seconds()),
	)

	return c.JSON(response)
}

// Глобальная структура для хранения последних значений метрик
func getCurrentStatusMetric(athleteID string) string {
	lastMetrics.RLock()
	defer lastMetrics.RUnlock()
	if vals, ok := lastMetrics.values[athleteID]; ok {
		if vals["status_active"] == 1 {
			return "active"
		}
	}
	return "resting"
}

// Получить все последние метрики для атлета
func getAllCurrentMetrics(athleteID string) fiber.Map {
	lastMetrics.RLock()
	defer lastMetrics.RUnlock()

	if vals, ok := lastMetrics.values[athleteID]; ok {
		result := fiber.Map{}
		for key, value := range vals {
			result[key] = value
		}
		return result
	}

	return fiber.Map{}
}

func simulateTrainingSessionHandler(c *fiber.Ctx) error {
	ctx := c.UserContext()

	var request struct {
		AthleteID string  `json:"athlete_id"`
		Duration  int     `json:"duration"` // в минутах
		Intensity float64 `json:"intensity" default:"0.7"`
	}

	if err := c.BodyParser(&request); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "invalid request body",
		})
	}

	if request.AthleteID == "" {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "athlete_id is required",
		})
	}

	if request.Duration <= 0 || request.Duration > 180 {
		request.Duration = 30
	}

	if request.Intensity <= 0 || request.Intensity > 1.0 {
		request.Intensity = 0.7
	}

	ctx, span := tracer.Start(ctx, "simulate-training-session",
		trace.WithAttributes(
			attribute.String("athlete.id", request.AthleteID),
			attribute.Int("duration_minutes", request.Duration),
			attribute.Float64("intensity", request.Intensity),
		))
	defer span.End()

	config := getAthleteConfigWithTracing(ctx, request.AthleteID)
	config.mu.Lock()
	config.IsTraining = true
	config.TrainingIntensity = request.Intensity
	config.mu.Unlock()

	var sessionData []SensorData

	// Симуляция тренировки с интервалом в 5 минут
	for minute := 0; minute < request.Duration; minute += 5 {
		//timeSinceUpdate := float64(minute)
		physioData := generatePhysiologicalDataWithTracing(ctx, config)
		activityData := generateActivityDataWithTracing(ctx, config)
		muscleLoad := generateMuscleLoadWithTracing(ctx, config, physioData.Fatigue)
		detectAnomaliesWithTracing(ctx, request.AthleteID, physioData)

		data := SensorData{
			AthleteID:        request.AthleteID,
			DeviceID:         fmt.Sprintf("garmin_%s", request.AthleteID[len(request.AthleteID)-3:]),
			HeartRate:        physioData.HeartRate,
			HeartRateVar:     physioData.HRV,
			BreathingRate:    physioData.BreathingRate,
			OxygenSaturation: physioData.Oxygen,
			Temperature:      physioData.Temperature,
			HydrationLevel:   physioData.Hydration,
			Steps:            activityData.Steps,
			Speed:            activityData.Speed,
			Distance:         activityData.Distance,
			Calories:         activityData.Calories,
			Fatigue:          physioData.Fatigue,
			MuscleLoad:       muscleLoad,
			TotalSteps:       config.TotalSteps + int64(activityData.Steps),
			Timestamp:        time.Now().Add(time.Duration(minute) * time.Minute).Format(time.RFC3339),
		}

		sessionData = append(sessionData, data)
		updateMetricsWithTracing(ctx, data)

		// Небольшая задержка для симуляции
		time.Sleep(50 * time.Millisecond)
	}

	// Завершаем тренировку
	config.mu.Lock()
	config.IsTraining = false
	config.LastUpdate = time.Now()
	config.mu.Unlock()

	sessionMetric.Inc()
	globalSessionID++

	span.SetAttributes(
		attribute.Int("data_points", len(sessionData)),
		attribute.Int64("session_id", globalSessionID),
	)

	return c.JSON(fiber.Map{
		"session_id":   globalSessionID,
		"athlete_id":   request.AthleteID,
		"duration":     request.Duration,
		"intensity":    request.Intensity,
		"data_points":  len(sessionData),
		"session_data": sessionData,
		"completed_at": time.Now().Format(time.RFC3339),
	})
}

// Обновляем в updateMetricsWithTracing
func updateMetricsWithTracing(ctx context.Context, data SensorData) {
	_, span := tracer.Start(ctx, "update-metrics")
	defer span.End()
	status := "resting"
	if data.HeartRate > 100 {
		status = "active"
	}
	heartRateMetric.WithLabelValues(data.AthleteID, status).Set(float64(data.HeartRate))
	fatigueMetric.WithLabelValues(data.AthleteID).Set(data.Fatigue)
	hydrationMetric.WithLabelValues(data.AthleteID).Set(data.HydrationLevel)
	oxygenMetric.WithLabelValues(data.AthleteID).Set(data.OxygenSaturation)
	temperatureMetric.WithLabelValues(data.AthleteID).Set(data.Temperature)
	stepsMetric.WithLabelValues(data.AthleteID).Add(float64(data.Steps))
	caloriesMetric.WithLabelValues(data.AthleteID).Add(float64(data.Calories))
	distanceMetric.WithLabelValues(data.AthleteID).Add(data.Distance)
	heartRateHistogram.WithLabelValues(data.AthleteID).Observe(float64(data.HeartRate))
	// Сохраняем последние значения
	lastMetrics.Lock()
	if lastMetrics.values[data.AthleteID] == nil {
		lastMetrics.values[data.AthleteID] = make(map[string]float64)
	}
	lastMetrics.values[data.AthleteID]["heart_rate"] = float64(data.HeartRate)
	lastMetrics.values[data.AthleteID]["fatigue"] = data.Fatigue
	lastMetrics.values[data.AthleteID]["hydration"] = data.HydrationLevel
	lastMetrics.values[data.AthleteID]["oxygen"] = data.OxygenSaturation
	lastMetrics.values[data.AthleteID]["temperature"] = data.Temperature
	lastMetrics.Unlock()
	span.SetAttributes(
		attribute.String("athlete.id", data.AthleteID),
		attribute.String("status", status),
	)
}

// Упрощенные функции для получения метрик
func getCurrentHeartRateMetric(athleteID string) float64 {
	lastMetrics.RLock()
	defer lastMetrics.RUnlock()
	if vals, ok := lastMetrics.values[athleteID]; ok {
		return vals["heart_rate"]
	}
	return 0
}

func getCurrentFatigueMetric(athleteID string) float64 {
	lastMetrics.RLock()
	defer lastMetrics.RUnlock()
	if vals, ok := lastMetrics.values[athleteID]; ok {
		return vals["fatigue"]
	}
	return 0
}

func getCurrentHydrationMetric(athleteID string) float64 {
	lastMetrics.RLock()
	defer lastMetrics.RUnlock()
	if vals, ok := lastMetrics.values[athleteID]; ok {
		return vals["hydration"]
	}
	return 0
}

func getCurrentOxygenMetric(athleteID string) float64 {
	lastMetrics.RLock()
	defer lastMetrics.RUnlock()
	if vals, ok := lastMetrics.values[athleteID]; ok {
		return vals["oxygen"]
	}
	return 0
}

func getCurrentTemperatureMetric(athleteID string) float64 {
	lastMetrics.RLock()
	defer lastMetrics.RUnlock()
	if vals, ok := lastMetrics.values[athleteID]; ok {
		return vals["temperature"]
	}
	return 0
}

func getTotalCaloriesMetric(_ string) float64 {
	// Для счетчиков используем глобальные переменные или просто возвращаем 0
	return 0
}

func getTotalDistanceMetric(_ string) float64 {
	return 0
}

func getAnomaliesCount(_ string) fiber.Map {
	// Упрощенный вариант
	return fiber.Map{
		"high_heart_rate": 0,
		"low_oxygen":      0,
		"dehydration":     0,
		"high_temp":       0,
		"high_fatigue":    0,
	}
}

func bToMb(b uint64) float64 {
	return float64(b) / 1024 / 1024
}

// ==================== MAIN FUNCTION ====================
func main() {
	// Инициализация трассировки
	tp, err := initTracer()
	if err != nil {
		log.Fatal(err)
	}
	defer func() {
		if err := tp.Shutdown(context.Background()); err != nil {
			log.Printf("Error shutting down tracer: %v", err)
		}
	}()
	app := fiber.New(fiber.Config{
		ReadTimeout:  10 * time.Second,
		WriteTimeout: 10 * time.Second,
	})
	// Глобальные middleware
	app.Use(tracingMiddleware)
	app.Use(func(c *fiber.Ctx) error {
		c.Set("X-Powered-By", "Athlete Monitoring API")
		c.Set("X-Version", "1.0.0")
		return c.Next()
	})
	app.Use(tracingDebugMiddleware)
	// Группа API роутов
	api := app.Group("/api")
	// Health check
	app.Get("/health", healthCheckHandler)
	// API endpoints
	api.Get("/sensor/:id", getSensorDataHandler)
	api.Post("/sensor/batch", getMultipleSensorDataHandler)
	api.Get("/stats/:id", getAthleteStatsHandler)
	api.Get("/metrics", getMetricsHandler)
	api.Post("/metrics/push", pushMetricsHandler)
	api.Post("/training/simulate", simulateTrainingSessionHandler)
	app.Get("/health-chain", healthCheckHandler)
	// Graceful shutdown
	go func() {
		sigChan := make(chan os.Signal, 1)
		signal.Notify(sigChan, os.Interrupt, syscall.SIGTERM)
		<-sigChan

		log.Println("🛑 Shutting down server...")
		if err := app.Shutdown(); err != nil {
			log.Fatal(err)
		}
	}()
	chaosCtx, chaosCancel := context.WithCancel(context.Background())
	defer chaosCancel()
	go startChaosTask(chaosCtx)
	// Автоматический пуш метрик каждые 30 секунд
	go func() {
		ticker := time.NewTicker(30 * time.Second)
		defer ticker.Stop()
		for range ticker.C {
			select {
			case <-ticker.C:
				ctx := context.Background()
				if err := pushMetricsWithTracing(ctx); err != nil {
					log.Printf("⚠️ Auto-push failed: %v", err)
				}
			default:
				{
				}
			}
		}
	}()
	log.Println("🚀 Server starting on :3000")
	log.Println("📊 Prometheus metrics available at :3000/api/metrics")
	log.Println("🔍 Jaeger tracing available at http://localhost:16686")
	if err := app.Listen(":3000"); err != nil {
		log.Fatal(err)
	}
}
