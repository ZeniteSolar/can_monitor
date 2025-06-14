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
            sin_wave_smooth = math.sin(t/100)
            sin_wave_delayed = math.sin(t-0.1)
            cos_wave = math.cos(t)
            
            mock_data = {
                "boat_on": True,
                "motor_on": t % 20 < 10,  # on for 10s, off for 10s
                "motor_rev": t % 5 < 2.5,
                "dms_on": t % 15 < 3,
                "pump": [bool(int((t + i) % 2)) for i in range(3)],

                "dir_bat_v": 13.0 + 0.1 * sin_wave,
                "dir_bat_i": 2.0 + 0.1 * cos_wave,
                "dir_bat_p": 25 + 2 * sin_wave,
                "dir_pos": [int(120 + 120 * sin_wave_delayed), int(120 + 120 * sin_wave)],

                
                "motor_d": [int(120 + 120 * sin_wave_delayed), int(120 + 120 * sin_wave)],
                "motor_rpm": 1500,

                "mic_machine_state": int(t) % 5,
                # "mcs_machine_state": int(t) % 5,
                "mam_machine_state": int(t) % 5,
                "mac_machine_state": int(t) % 5,
                "msc_machine_state": [int(t) % 5, int(t) % 5, int(t) % 5, int(t) % 5, int(t) % 5],
                "mcb_machine_state": [int(t) % 5, int(t) % 5],
                "mde_machine_state": int(t) % 5,

                "mic_error_code": int(t) % 8,
                "mcs_error_code": int(t+1) % 8,
                "mam_error_code": int(t+2) % 8,
                "mac_error_code": int(t+3) % 8,
                "msc_error_code": [1, 2, 3, 4, 5],
                "mcb_error_code": [1, 2],
                "mde_error_code": int(t+5) % 8,

                "bat_v": 3.8 + 0.2 * sin_wave,
                "bat_cell_v": [12.5, 12.2, 12.7],
                "bat_ii": 3.0 + 0.5 * cos_wave,
                "bat_io": 1.0 + 0.1 * sin_wave,
                "bat_i": 3.5 + 0.3 * cos_wave,
                "bat_p": 36 + 3 * sin_wave,

                "mcb_d": [0.1 + 0.05 * sin_wave, 0.2 + 0.05 * cos_wave],
                "mcb_vi": [3*12.6 + 0.1 * cos_wave, 3*13],
                "mcb_io": [1.1 + 0.1 * sin_wave, 1.2],
                "mcb_vo": [11.8 + 0.1 * cos_wave, 11.9],
                "mcb_po": [13.0 + 0.5 * sin_wave, 14.0],

            }
            
            
            # Omit `mic_machine_state` for 6s every 20s
            if t % 10 >= 4:
                mock_data["mcs_machine_state"] = int(t) % 5

            await ws.send(json.dumps(mock_data))
            print("✅ Sent mock BoatData frame")
            await asyncio.sleep(0.05)

asyncio.run(send_mock_boat_data())