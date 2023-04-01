import fs from "fs";

interface BundleFile {
    name: string
    data: Buffer
}

export const getBundleFor = (fileName: string): Set<BundleFile> => {
    fileName = fileName.replace(/\\/g, "/");
    const file: BundleFile | null = (() => {
        try {
            return { data: fs.readFileSync(`${process.env.LDRAW_LIB}/parts/${fileName}`), name: fileName }
        } catch (e) {
            try {
                return { data: fs.readFileSync(`${process.env.LDRAW_LIB}/p/${fileName}`), name: fileName }
            } catch (e) {
                return null;
            }
        }
    })();

    if (file === null) return new Set();

    const lines = file.data.toString().split(/\r?\n/).filter(line => line.trim().charAt(0) === '1');
    const subfiles = new Set(lines.map(line => {
        let tokens = line.split(/\t| /gm);
        return tokens[tokens.length - 1];
    }));

    return new Set([file, ...Array.from(subfiles).map(f => getBundleFor(f)).reduce((acc, x) => new Set([...acc, ...x]), new Set())]);
}