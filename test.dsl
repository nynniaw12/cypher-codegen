# Node definitions
NODE Person {
  id: string,
  name: string,
  age: integer
}

NODE Movie {
  id: string,
  title: string,
  year: integer
}

# Relationship definition
RELATIONSHIP (Person)-ACTED_IN->(Movie) {
    role: string,
    salary: float
}

# Node endpoints
ENDPOINT Person /api/person {
  CREATE
}

ENDPOINT Person /api/person/:personId (match: {id: personId}) {
  READ
  UPDATE
  DELETE
}

# Relationship endpoints
ENDPOINT ACTED_IN /api/person/:personId/acted-in/:movieId (match: {from.id: personId, to.id: movieId}) {
  CREATE
  READ
  UPDATE
  DELETE
}

ENDPOINT ACTED_IN /api/person/:personId/movies (match: {from.id: personId}) {
  READ-TO
}

ENDPOINT ACTED_IN /api/movie/:movieId/actors (match: {to.id: movieId}) {
  READ-FROM
}
