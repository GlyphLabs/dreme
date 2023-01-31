# dreme

a simple (and really fast!) rest api that returns memes from reddit

## endpoints


### get a meme
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
    "title": "Cultural Appropriation",
    "author": "GalfridusMagnus",
    "subreddit": "dankmemes",
    "post_ink": "https://reddit.com/r/dankmemes/comments/10ot7g5/cultural_appropriation/",
    "ups": 559,
    "image_url": "https://i.redd.it/j2pb4gop34fa1.png"
  },
  {
    ...
  },
]
```
### error handling
if the specified subreddit does not exist, the API will return a 400 error.
if reddit returns an invalid response or something goes wrong during parsing, the api will return a 500 error.

## todo
- [ ] add better error handling
