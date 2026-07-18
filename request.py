#!/usr/bin/env python3

from dataclasses import dataclass
from enum import Enum
from typing import ByteString, Dict


class HTTPMethod(Enum):
    GET = "GET"
    POST = "POST"
    DELETE = "DELETE"

@dataclass
class Request:
    method: HTTPMethod
    headers: Dict[str, str]
    path: str
    body: ByteString



def parse_request(request_string: str) -> Request:
    parts = request_string.split("\r\n")

    request_line = parts[0]

    method = HTTPMethod(request_line.split()[0])
    path = request_line.split()[1]

    index = 1
    headers = {}

    while parts[index] != "":
        header = parts[index]

        key = header.split(":")[0]
        value = header.split(":")[1].lstrip()

        headers.setdefault(key, value)

        index += 1


    body = b''
    if headers.get("Content-Length") != None and method != "GET":
        index += 1 # skip the \n char

        while index < len(parts): # pyright: ignore
            body += parts[index].encode()
            index += 1

    req = Request(method, headers, path, body)

    return req
