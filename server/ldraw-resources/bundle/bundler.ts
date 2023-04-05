import fs from "fs";

interface BundleFile {
    name: string
    data: Buffer
}

export const init = () => {
    if (!fs.existsSync(process.env.ZIP_DIR ?? "./zip")) {
        fs.mkdirSync(process.env.ZIP_DIR ?? "./zip");
    }

    if (!fs.existsSync(process.env.LST_DIR ?? "./lst")) {
        fs.mkdirSync(process.env.LST_DIR ?? "./lst");
    }
}

export const getBundleFor = (fileName: string): Set<BundleFile> => {
    fileName = fileName.replace(/\\/g, "/");
    const file = getFile(fileName)

    if (file === null) return new Set();

    const lines = file.data.toString().split(/\r?\n/).filter(line => line.trim().charAt(0) === '1');
    const subfiles = new Set(lines.map(line => {
        let tokens = line.split(/\t| /gm);
        return tokens[tokens.length - 1];
    }));

    return new Set([file, ...Array.from(subfiles).map(f => getBundleFor(f)).reduce((acc, x) => new Set([...acc, ...x]), new Set())]);
}


export const getListFor = (fileName: string): Set<String> => {
    fileName = fileName.replace(/\\/g, "/");
    const file = getFile(fileName);
    if (file === null) return new Set();

    const lines = file.data.toString().split(/\r?\n/gm).filter(line => line.trim().charAt(0) === '1');
    const subfiles = new Set(lines.map(line => {
        let tokens = line.split(/\t| /gm);
        return tokens[tokens.length - 1];
    }));

    return new Set([file.name, ...Array.from(subfiles).map(f => getListFor(f)).reduce((acc, x) => new Set([...acc, ...x]), new Set())]);
}

export const getPrebuildBundleFor = (id: string) => {
    const file = `${process.env.ZIP_DIR}/${id}.zip`;
    if (!fs.existsSync(file)) return null;
    return fs.createReadStream(file);
}

export const getPrebuildListFor = (id: string) => {
    const file = `${process.env.LST_DIR}/${id}.lst`;
    if (!fs.existsSync(file)) return null;
    return fs.createReadStream(file);
}

const getFile = (fileName: string): BundleFile | null => {
    try {
        return { data: fs.readFileSync(`${process.env.LDRAW_LIB}/parts/${fileName}`), name: fileName }
    } catch (e) {
        try {
            return { data: fs.readFileSync(`${process.env.LDRAW_LIB}/p/${fileName}`), name: fileName }
        } catch (e) {
            return null;
        }
    }
}