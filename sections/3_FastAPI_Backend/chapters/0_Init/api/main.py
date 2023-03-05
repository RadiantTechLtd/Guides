from fastapi import FastAPI


app = FastAPI()


@app.get("/")
async def splashpage():
    return f"Hello, World!"
