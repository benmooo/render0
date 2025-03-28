# Render0 - a tiny software renderer in rust

1. tiny renderer inspired software rasterization renderer based on softbuffer and winit

2. it's all about pixels

- draw line (Bresenham's line drawing algorithm)
- triangle resterization & back face culling
- z buffer (hidden faces removal)
- perspective projection
- moving the camera
- shaders for the software renderer
- tangent space normal mapping
- shadow mapping
- ambient occlusion

```py
import matplotlib.pyplot as plt
from itertools import accumulate
from typing import List
import numpy as np


def rise_by(rate: float):
    return lambda x: x * (1 + rate)

def decline_by(rate: float):
    return lambda x: x * (1 - rate)

def rises(rises: List[float]):
    return lambda x: List(accumulate(rises, lambda acc, r: rise_by(r)(acc), initial=x))

def declines(declines: List[float]):
    return lambda x: List(accumulate(declines, lambda acc, r: decline_by(r)(acc), initial=x))

def fluctuate(ratio: float):
    return lambda x : x * ratio

def fluctuations(functuations: List[float]):
    return lambda x: List(accumulate(functuations, lambda acc, f: f(acc), initial=x))


def rises_then_declines(rises: List[float], declines: List[float]):
    def rise_then_decline_func(x):
        r = List(accumulate(rises, lambda acc, r: rise_by(r)(acc), initial=x))


def draw_stock_rise_and_decline(history: list):
    # we draw the amount using plt
    plt.plot(history)
    plt.show()


def main():
    # we draw the jmount using plt
    # generate random history using numpy and clamp value between 0 and 1
    # history = np.random.uniform(0, 1, 20)
    # plt.plot(history)
    # plt.show()

    # generate a list of the same value for 20 times use numpy
    history = np.full(500, 0.9)
    print(history)

main()

```

![render0](https://upload.wikimedia.org/wikipedia/commons/d/d1/Rendering_eq.png)
