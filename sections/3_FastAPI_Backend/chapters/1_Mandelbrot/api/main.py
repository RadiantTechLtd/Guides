from fastapi import FastAPI

import mandybrot as mandy


app = FastAPI()


@app.get("/")
async def splashpage():
    return f"Hello, World!"


@app.get("/sample/{real}/{imag}")
async def sample_point(real: float, imag: float):
    max_iters = 100
    result = mandy.sample.point(real, imag, max_iters)
    return f"({real}, {imag}) -> {result}"
