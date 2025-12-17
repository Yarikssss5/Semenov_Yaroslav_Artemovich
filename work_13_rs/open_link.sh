MYUSER=$(whoami) \
    && MYUID=$(id --user $MYUSER) \
    && DISPLAY=:0  \
    && XAUTHORITY=/home/$MYUSER/.Xauthority \
    && DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/$MYGID/bus \ 
    && xdg-open https://rutube.ru/video/9c2ee5c79ce9f0ae21b58ed977466c38/?r=plwd