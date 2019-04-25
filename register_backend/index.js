const express = require('express');
const app = express();
const port = 3000;

const apiResponse = {
    _base: () => {
        return ({
            time: Date.now().toString()
        });
    },
    success: (data) => {
        return ({
            ...this._base(),
            data
        });
    },
    error: (code, message) => {
        return ({
            ...this._base(),
            err: {
                message,
                code
            }
        });
    }
};

app.get('/', (req, res) => res.send('Hello World!'));
app.get('/registration-details/:uuid', (req, res) => {
    const uuid = req.params.uuid;
    res.send(apiResponse.success({
        uuid,
        firstname: "Dennis",
        lastname: "Kievits"
    }));
});

app.listen(port, () => console.log(`Example app listening on port ${port}!`));