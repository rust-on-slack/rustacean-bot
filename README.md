# Rustacean bot
The friendly bot used on https://rust-slack.herokuapp.com

## Running
```bash
cargo run
```
And the server will be running at: `http://localhost:3000`

## Testing
Running tests
```bash
cargo test
cargo test -- --ignored # Runs the integration tests
```

## Deploying to Heroku

```
$ heroku create
$ git push heroku master
$ heroku open
```
or

[![Deploy to Heroku](https://www.herokucdn.com/deploy/button.png)](https://heroku.com/deploy)

# LICENSE
MIT
