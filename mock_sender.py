import asyncio
import websockets
import json
import math
import time
import random
import sys

# Get IP from argument or use default
ip = sys.argv[1] if len(sys.argv) > 1 else "localhost"
uri = f"ws://{ip}:3001"

async def send_mock_boat_data():
    start_time = time.time()

    async with websockets.connect(uri, ping_interval=None) as ws:
        print("Connected to", uri)

        while True:
            t = time.time() - start_time
            # Oscillating value with noise
            sin_wave = math.sin(t)
            sin_wave_smooth = math.sin(100*t)
            cos_wave = math.cos(t)

            mock_data = {
                "boat_on": True,
                "motor_on": t % 20 < 10,  # on for 10s, off for 10s
                "motor_rev": t % 5 < 2.5,
                "dms_on": t % 15 < 3,
                "pump": [bool(int((t + i) % 2)) for i in range(3)],

                "motor_d": [abs(sin_wave), abs(cos_wave)],
                "motor_rpm": 1500 + 200 * sin_wave,

                "mam_machine_state": int(t / 3) % 5,
                "mic_machine_state": int(t / 2) % 5,
                "mcs_machine_state": int(t / 3) % 5,
                "mac_machine_state": int(t / 4) % 4,
                "mde_machine_state": int(t / 5) % 4,

                "mcb_machine_state": [int(t) % 2, (int(t / 2) + 1) % 2],

                "mam_error_code": 0,
                "mic_error_code": 0,
                "mcs_error_code": 0,
                "mac_error_code": 0,
                "mde_error_code": 0,
                "mcb_error_code": [0, 0],

                "bat_v": 3.8 + 0.2 * sin_wave,
                "bat_cell_v": [12.5, 12.2, 12.7],
                "bat_ii": 3.0 + 0.5 * cos_wave,
                "bat_io": 1.0 + 0.1 * sin_wave,
                "bat_i": 3.5 + 0.3 * cos_wave,
                "bat_p": 36 + 3 * sin_wave,

                "dir_bat_v": 13.0 + 0.1 * sin_wave,
                "dir_bat_i": 2.0 + 0.1 * cos_wave,
                "dir_bat_p": 25 + 2 * sin_wave,
                "dir_pos": [0 + 15 * sin_wave, 15 * cos_wave],

                "mcb_d": [0.1 + 0.05 * sin_wave, 0.2 + 0.05 * cos_wave],
                "mcb_vi": [12.5 + 0.1 * cos_wave, 12.6],
                "mcb_io": [1.1 + 0.1 * sin_wave, 1.2],
                "mcb_vo": [11.8 + 0.1 * cos_wave, 11.9],
                "mcb_po": [13.0 + 0.5 * sin_wave, 14.0],

            }

            await ws.send(json.dumps(mock_data))
            print("✅ Sent mock BoatData frame")
            await asyncio.sleep(0.05)

asyncio.run(send_mock_boat_data())