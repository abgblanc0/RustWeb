/*
async function showUsers() {
    let body = document.getElementById("users")
    let out = ""
    const response = await fetch("http://127.0.0.1:8000/api/users")
                            .then(r => r.json())
                            .then(users => {
                                console.log(users)
                                for(let user of users) {
                                    out += 
                                    `
                                    <h1> ${user.username} <h1>
                                    <h2> ${user.password} <h2>
                                    `
                                }
                            })

    body.innerHTML = out
}

showUsers()
*/