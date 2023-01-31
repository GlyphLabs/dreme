# ☁️ dreme

a simple (and really fast!) rest api that returns memes from reddit

## features
- ⚡ fast (like, faster than reddit)
- 📦 small (responses are minimized to save time and bandwith)
- 🪴 simple (just one endpoint)


## 🌐 endpoints


### 💀 get a meme
`GET` `/[subreddit]`
gets a meme from the specified subreddit. if none is specified, it gets one from a random subreddit. you can also specify the amount of memes you want to retrieve by adding the `amount` query param.

#### parameters
`subreddit` (optional) - the subreddit from which to retrieve the meme.

`amount` (optional) - the number of memes to retrieve. default is 1.

#### example
get one meme from the dankmemes subreddit:

`GET` `/dankmemes`

get three memes from the dankmemes subreddit:

`GET` `/dankmemes?amount=3`

```
HTTP 200 OK

[
  {
		"title": "Get the stretcher",
		"author": "crankbot2000",
		"subreddit": "dankmemes",
		"permalink": "/r/dankmemes/comments/10q4zbx/get_the_stretcher/",
		"ups": 304,
		"url": "https://i.redd.it/1vpqt2miggfa1.gif"
	},
  {
    ...
  },
]
```
### 🥅 error handling
if the specified subreddit does not exist, the API will return a 400 error.
if reddit returns an invalid response or something goes wrong during parsing, the api will return a 500 error.

## 💭 todo
- [ ] add better error handling
