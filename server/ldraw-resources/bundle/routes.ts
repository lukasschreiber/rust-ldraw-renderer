import express, { Express, Request, Response } from 'express';
import { getBundleFor, getPrebuildBundleFor } from './bundler';
import JSZip from "jszip";
import fs from "fs";

export const BundleRouter = express.Router();

BundleRouter.get('/:id', async (req: Request, res: Response) => {
    const zipName = `${req.params.id}.zip`;
    const entryFileName = `${req.params.id}.dat`;

    const zipHeaders = {
        'Content-Type': 'application/zip',
        'Content-disposition': `attachment; filename=${zipName}`
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
