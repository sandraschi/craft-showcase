#!/usr/bin/env python3
"""Multi-threaded static HTTP server for craft-showcase with Range (206) support."""
import argparse
import functools
import os
import re
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer


class QuietRangeHandler(SimpleHTTPRequestHandler):
    def log_message(self, format: str, *args: object) -> None:
        pass

    def handle_one_request(self) -> None:
        try:
            super().handle_one_request()
        except (BrokenPipeError, ConnectionResetError, ConnectionAbortedError):
            self.close_connection = True

    def send_head(self):
        path = self.translate_path(self.path)
        if os.path.isdir(path) or not os.path.isfile(path):
            return super().send_head()
        range_header = self.headers.get("Range")
        if not range_header:
            self._range = None
            return super().send_head()
        m = re.match(r"bytes=(\d*)-(\d*)", range_header.strip())
        if not m:
            self._range = None
            return super().send_head()
        size = os.path.getsize(path)
        start_str, end_str = m.group(1), m.group(2)
        if start_str == "" and end_str == "":
            self._range = None
            return super().send_head()
        if start_str == "":
            length = int(end_str)
            start = max(0, size - length)
            end = size - 1
        else:
            start = int(start_str)
            end = int(end_str) if end_str else size - 1
        if start >= size or end < start:
            self.send_error(416, "Requested Range Not Satisfiable")
            return None
        end = min(end, size - 1)
        f = open(path, "rb")
        f.seek(start)
        self._range = (start, end)
        ctype = self.guess_type(path)
        self.send_response(206)
        self.send_header("Content-type", ctype)
        self.send_header("Accept-Ranges", "bytes")
        self.send_header("Content-Range", f"bytes {start}-{end}/{size}")
        self.send_header("Content-Length", str(end - start + 1))
        self.send_header("Last-Modified", self.date_time_string(os.path.getmtime(path)))
        self.end_headers()
        return f

    def copyfile(self, source, outputfile):
        rng = getattr(self, "_range", None)
        if rng is None:
            return super().copyfile(source, outputfile)
        remaining = rng[1] - rng[0] + 1
        bufsize = 64 * 1024
        while remaining > 0:
            chunk = source.read(min(bufsize, remaining))
            if not chunk:
                break
            outputfile.write(chunk)
            remaining -= len(chunk)


def main() -> None:
    parser = argparse.ArgumentParser(description="Multi-threaded craft-showcase server")
    parser.add_argument("--port", type=int, default=11270)
    parser.add_argument("--bind", default="127.0.0.1")
    parser.add_argument("--directory", default="web")
    args = parser.parse_args()

    os.chdir(args.directory)
    handler = functools.partial(QuietRangeHandler, directory=os.getcwd())
    server = ThreadingHTTPServer((args.bind, args.port), handler)
    server.daemon_threads = True
    print(f"craft-showcase ThreadingHTTPServer listening on http://{args.bind}:{args.port}", flush=True)
    server.serve_forever()


if __name__ == "__main__":
    main()
