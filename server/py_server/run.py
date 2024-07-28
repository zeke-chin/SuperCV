import argparse
import uvicorn

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--host', default='0.0.0.0')
    parser.add_argument('--port', default=8000)
    parser.add_argument('--debug', default=False)
    opt = parser.parse_args()

    app_str = 'server:app'  # make the app string equal to whatever the name of this file is
    uvicorn.run(app_str, host=opt.host, port=int(opt.port), timeout_keep_alive=60, reload=opt.debug)
