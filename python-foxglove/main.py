import time

import foxglove
from foxglove.channels import Vector3Channel
from foxglove.schemas import Vector3

file_name = "test_recording.mcap"
writer = foxglove.open_mcap(file_name)
server = foxglove.start_server()


while True:
    location_channel = Vector3Channel("/location")
    location_channel.log(Vector3(x=1, y=2, z=3))
    foxglove.log("/hello", {"time": time.time()})
    time.sleep(0.03)
