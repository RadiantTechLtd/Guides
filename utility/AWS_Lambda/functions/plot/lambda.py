import json
import base64

import mandybrot as mandy


def handler(event, context):
    body = unwrap_payload(event)
    print(body)
    return response()


def unwrap_payload(event):
    if "body" not in event:
        raise Exception("No body in request.")
    body = event["body"]
    if "isBase64Encoded" in event:
        if event["isBase64Encoded"]:
            body = base64.b64decode(body)
    try:
        payload = json.loads(body)
    except:
        raise Exception("Could not parse body as JSON.")
    return payload


def response():
    status = 200  # 200 = OK
    headers = {
        # "Access-Control-Allow-Headers": "Content-Type",
        "Access-Control-Allow-Origin": "*",
        # "Access-Control-Allow-Credentials": "true",
        # "Access-Control-Expose-Headers": "x-api-id",
        # "Access-Control-Max-Age": "300",
        # "Access-Control-Allow-Methods": "*",
    }
    body = {"message": "Request processed."}
    return {"statusCode": status, "headers": headers, "body": json.dumps(body)}
