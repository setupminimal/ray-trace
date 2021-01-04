#!/usr/bin/env python3

import os
from tqdm import tqdm

framerate = 6

for i, t in tqdm(list(enumerate(range(3620*framerate, 4500*framerate))), desc="Shooting"):
    os.system(f'./target/release/ray-trace -t {t/framerate} -f movie/{i:05}.ppm -x 1280 -y 800 -s 150')

os.system(f'ffmpeg -f image2 -r {framerate} -i "movie/%05d.ppm" -vcodec libx264 -profile:v high444 -crf 0 -preset veryslow -y out.mp4')
