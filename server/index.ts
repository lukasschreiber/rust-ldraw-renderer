import express, { Express, Request, Response } from 'express';
import dotenv from 'dotenv';
import { getBundleFor, getPrebuildBundleFor } from './bundler';
import JSZip from "jszip";
import fs from "fs";
import cors from "cors";

dotenv.config();

if (!fs.existsSync(process.env.ZIP_DIR ?? "./zip")) {
    fs.mkdirSync(process.env.ZIP_DIR ?? "./zip");
}

const app: Express = express();
const port = process.env.PORT;

app.use(cors())

app.get('/', (req: Request, res: Response) => {
    res.send("test")
})

app.get('/bundle/:id', async (req: Request, res: Response) => {
    const zipName = `${req.params.id}.zip`;
    const entryFileName = `${req.params.id}.dat`;

    const zipHeaders = {
        'Content-Type': 'application/zip',
        'Content-Disposition': 'inline',
        // 'Content-disposition': `attachment; filename=${zipName}`
    }

    const prebuildBundle = getPrebuildBundleFor(req.params.id);
    if (prebuildBundle !== null) {
        res.writeHead(200, zipHeaders);

        return prebuildBundle.pipe(res);
    }

    const bundle = Array.from(getBundleFor(entryFileName));

    if (bundle.length > 0 && req.params.id !== null) {
        const zip = new JSZip();
        bundle.forEach(file => zip.file(file.name, file.data));

        res.writeHead(200, zipHeaders);

        // if storage space is a concern comment this out and all bundles are calculated on the fly
        zip.generateNodeStream({ streamFiles: true })
            .pipe(fs.createWriteStream(`${process.env.ZIP_DIR}/${zipName}`));

        zip.generateNodeStream({ streamFiles: true })
            .pipe(res)
    } else {
        res.sendStatus(404)
    }
});

app.listen(port, () => {
    console.log(`⚡️[server]: Server is running at http://localhost:${port}`);
});