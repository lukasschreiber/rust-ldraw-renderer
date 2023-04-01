import express, { Express, Request, Response } from 'express';
import dotenv from 'dotenv';
import { getBundleFor } from './bundler';
import JSZip from "jszip";

dotenv.config();

const app: Express = express();
const port = process.env.PORT;

app.get('/bundle/:id', async (req: Request, res: Response) => {
    const entryFileName = `${req.params.id}.dat`;
    const bundle = Array.from(getBundleFor(entryFileName));

    if (bundle.length > 0 && req.params.id !== null) {
        const zip = new JSZip();
        bundle.forEach(file => zip.file(file.name, file.data));

        res.writeHead(200, {
            'Content-Type': 'application/zip',
            'Content-disposition': `attachment; filename=${req.params.id}.zip`
        });

        zip.generateNodeStream({ streamFiles: true })
            .pipe(res);
    } else {
        res.sendStatus(404)
    }
});

app.listen(port, () => {
    console.log(`⚡️[server]: Server is running at http://localhost:${port}`);
});