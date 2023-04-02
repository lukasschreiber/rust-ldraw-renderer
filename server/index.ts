import express, { Express, Request, Response } from 'express';
import dotenv from 'dotenv';
import cors from "cors";
import { LDRAWRouter } from './ldraw-resources/static-files';

dotenv.config();

const app: Express = express();
const port = process.env.PORT;

app.use(cors())

app.use("/ldraw", LDRAWRouter)

app.get('/', (req: Request, res: Response) => {
    res.send("test")
})


app.listen(port, () => {
    console.log(`⚡️[server]: Server is running at http://localhost:${port}`);
});