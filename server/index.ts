import express, { Express, Request, Response } from 'express';
import dotenv from 'dotenv';
import { getBundleFor, getPrebuildBundleFor } from './bundler';
import JSZip from "jszip";
import fs from "fs";

dotenv.config();

const app: Express = express();
const port = process.env.PORT;

if (!fs.existsSync(process.env.ZIP_DIR ?? "./zip")) {
    fs.mkdirSync(process.env.ZIP_DIR ?? "./zip");
}

app.get('/bundle/:id', async (req: Request, res: Response) => {
    const zipName = `${req.params.id}.zip`;
    const entryFileName = `${req.params.id}.dat`;

    const prebuildBundle = getPrebuildBundleFor(req.params.id);
    if (prebuildBundle !== null) {
        res.writeHead(200, {
            'Content-Type': 'application/zip',
            'Content-disposition': `attachment; filename=${zipName}`
        });

        return prebuildBundle.pipe(res);
    }

    const bundle = Array.from(getBundleFor(entryFileName));

    if (bundle.length > 0 && req.params.id !== null) {
        const zip = new JSZip();
        bundle.forEach(file => zip.file(file.name, file.data));

        res.writeHead(200, {
            'Content-Type': 'application/zip',
            'Content-disposition': `attachment; filename=${zipName}`
        });

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