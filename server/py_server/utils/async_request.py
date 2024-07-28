import aiohttp


async def post_llama_cpp(url, data_json):
    async with aiohttp.ClientSession() as session:
        async with session.post(url, json=data_json) as response:
            return await response.json()  # 直接返回response的json内容
