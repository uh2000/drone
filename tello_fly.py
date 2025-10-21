from djitellopy import Tello
tello = Tello()
tello.connect()
tello.takeoff()

tello.move_right(200)
tello.rotate_counter_clockwise(180)
tello.move_forward(150)

tello.land()