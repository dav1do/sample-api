
# Sample API

## Local 
`cargo run` will host on 127.0.0.1:8080

`127.0.0.1:8080/playground` -> get the playground
`127.0.0.1:8080/graphql` -> execute requests against the api

## Sample Requests

```
mutation signup {
  signup(input:{email:"david@test.com", password: "qwerty", name: "david"}) {
    name
    email
  }
}

mutation login {
  login(input: {email: "david@test.com", password:"qwerty"}) {
    token
  }
}

query favs($token: String) {
  getUser(input: {token:$token}) {
    name
    email
    favoriteCities {
      name
      country
    }
  }
}

mutation addCity($token: String!) {
  addFavoriteCity(
    input: { token: $token, name: "Minneapolis", country: "USA" }
  ) {
    city {
      name
      country
    }
  }
}

mutation removeCity($token: String!) {
  removeFavoriteCity(input: { token: $token, name: "Minneapolis", country: "USA" }) {
    success
  }
}

```

## Tasks
Should be able to accomplish the following:

    * Create a user account with the following:
    
        * Name
        
        * Email
        
        * Password
        
    * A user should be able to authenticate with an email and password
        
    * An authenticated user should be able to add and remove their favorite cities where a city consists of the following:
    
        * City name
        
        * Country
        
    * An authenticated user should be able to retrieve a list of their favorite cities
