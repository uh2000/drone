import time
from djitellopy import Tello

tello = Tello()
try:
    tello.connect()
    battery = tello.get_battery()
    print(f"Battery: {battery}%")
    if battery < 20:
        print("Battery too low for takeoff. Please charge.")
        print(f"Battery is {battery}")
    else:
        print(f"Battery is {battery}")
        # tello.takeoff()
        # time.sleep(3)  # let it stabilize briefly
        # tello.land()
finally:
    # Ensures sockets/SDK session are closed so the drone exits command mode
    tello.end()