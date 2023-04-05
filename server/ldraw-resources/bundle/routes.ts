import express, { Express, Request, Response } from 'express';
import { getBundleFor, getListFor, getPrebuildBundleFor, getPrebuildListFor, init } from './bundler';
import JSZip from "jszip";
import fs from "fs";

export const BundleRouter = express.Router();

init();

BundleRouter.get('/:id\.:v', async (req: Request, res: Response) => {
    if (req.params.v === "zip") {
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
    } else if (req.params.v === "lst") {
        const entryFileName = `${req.params.id}.dat`;

        const prebuildBundle = getPrebuildListFor(req.params.id);
        if (prebuildBundle !== null) {
            res.writeHead(200);

            return prebuildBundle.pipe(res);
        }

        const bundle = Array.from(getListFor(entryFileName));

        if (bundle.length > 0 && req.params.id !== null) {
            const fileContents = bundle.join("\r\n");
            res.setHeader("content-type", "text/plain");
            res.setHeader("content-disposition", `form-data; attachment; filename=${req.params.id}.lst"`)
            res.send(fileContents);
            fs.writeFileSync(`${process.env.LST_DIR}/${req.params.id}.lst`, fileContents);
        } else {
            res.sendStatus(404)
        }
    } else {
        res.sendStatus(404)
    }
});
