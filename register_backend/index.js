const express = require('express');
const app = express();
const port = 3000;

const baseApiResponse = () => {
    return ({
        time: Date.now().toString()
    });
};

const apiResponse = {
    success: (data) => {
        return ({
            ...baseApiResponse(),
            data
        });
    },
    error: (code, message) => {
        return ({
            ...baseApiResponse(),
            err: {
                message,
                code
            }
        });
    }
};

const apiErrorCodes = {
    USER_ALREADY_REGISTERED: 0x1,
    INVALID_REGISTRATION_ID: 0x2
};

app.get('/', (req, res) => res.send('Hello World!'));
app.get('/registration-details/:uuid', (req, res) => {
    const uuid = req.params.uuid;

    if (uuid === "5fc415af-00df-4269-b019-0670b96b71ec") {
        res.send(apiResponse.success({
            uuid,
            firstname: "Dennis",
            lastname: "Kievits"
        }));
        return;
    }
    if (uuid === "ffc9540f-8ae6-41ec-83cc-9f422e948bfc") {
        res.send(apiResponse.error(apiErrorCodes.USER_ALREADY_REGISTERED, "User already registered"));
        return;
    }

    res.send(apiResponse.error(apiErrorCodes.INVALID_REGISTRATION_ID, "Invalid registration id"));
});

app.listen(port, () => console.log(`Example app listening on port ${port}!`));