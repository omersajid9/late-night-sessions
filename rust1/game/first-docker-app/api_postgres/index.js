const keys = require('./keys')

const express = require('express');
const bodyParser = require('body-parser');
const cors = require('cors');

const app = express();
app.use(cors());
app.use(bodyParser.json())

const {Pool} = require('pg');
const pgClient = new Pool(
    {
        user: keys.pgUser,
        host: keys.pgHost,
        database: keys.pgDatabase,
        password: keys.pgPassword,
        port: keys.pgPort
    }
);

pgClient.on("connect", (client) =>
{
    console.log("DATRABASE CONNECTED")
    client.query("CREATE TABLE IF NOT EXISTS tasks (id SERIAL PRIMARY KEY, task VARCHAR(255))")
        .catch(err => console.log(err));
});

app.get('/', (req, res) =>
{
    res.send("Hi");
});

app.get('/values/all', async (req, res) =>
{
    const values = await pgClient.query("SELECT * FROM tasks");

    res.send(values);
});

app.post('/values', async (req, res) =>
{
    if (!req.body.value) res.send({working: false});
    pgClient.query("INSERT INTO tasks(task) VALUES($1)", [req.body.value])
    res.send({working: true});
});

app.delete('/delete/:taskId', (req, res) =>
{
    const taskId = req.params.taskId;
    const deleteQuery = "DELETE FROM tasks WHERE id = $1";
    pgClient.query(deleteQuery, [taskId]);
    res.send({success: true})
})

app.listen(5000, (err) =>
{
    console.log("Listening to 5000")
})