import time
from typing import ParamSpec
from typing_extensions import Self
from fastapi import FastAPI, Request, Response
from starlette.middleware.base import BaseHTTPMiddleware
import prometheus_client
from prometheus_client import CollectorRegistry, make_asgi_app, Summary, Counter, Gauge, Histogram
import uvicorn

app = FastAPI()
REGISTRY = CollectorRegistry()


class MyMetrics:
    requests_counter = Counter('my_http_requests_total', 'Total HTTP requests', ['method', 'endpoint'], registry=REGISTRY)
    request_duration = Histogram('my_http_request_duration_seconds', 'HTTP request duration', ['duration'], registry=REGISTRY)
    active_users = Gauge('my_active_users', 'Currently active users', registry=REGISTRY)
    request_size = Summary('my_request_size', 'Request size', registry=REGISTRY)


class MyMiddleware():

    def __init__(self):
        pass

    async def __call__(self, request: Request, call_next):
        MyMetrics.active_users.inc()
        MyMetrics.requests_counter.labels(method=str(request.method), endpoint=str(request.url)).inc()
        MyMetrics.request_size.observe(request.__sizeof__())
        start_time: float = time.monotonic()
        response = await call_next(request)
        end_time: float = time.monotonic()
        time_took: float = end_time - start_time
        MyMetrics.request_duration.labels(duration=str(time_took)).observe(time_took)
        MyMetrics.active_users.dec()
        return response



app.add_middleware(BaseHTTPMiddleware, dispatch=MyMiddleware())


@app.get("/metrics")
async def metrics_handler():
    return Response(
        prometheus_client.generate_latest(REGISTRY),
        media_type="text/plain; version=0.0.4"
    )

    

if __name__ == "__main__":
    uvicorn.run(app=app, host="localhost", port=3000)
