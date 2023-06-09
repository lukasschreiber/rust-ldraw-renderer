import express, { Router } from "express";
import { BundleRouter } from "./bundle/routes";
import dotenv from "dotenv";

dotenv.config();

export const LDRAWRouter = Router();

LDRAWRouter.use("/bundle", BundleRouter)

LDRAWRouter.use("/data/parts", express.static(`${process.env.LDRAW_LIB}/parts`));
LDRAWRouter.use("/data/parts", express.static(`${process.env.LDRAW_LIB}/p`));

LDRAWRouter.use("/config/LDConfig.ldr", express.static(`${process.env.LDRAW_LIB}/LDConfig.ldr`));
LDRAWRouter.use("/license/CAlicense.txt", express.static(`${process.env.LDRAW_LIB}/CAlicense.txt`));
LDRAWRouter.use("/license/CAlicense4.txt", express.static(`${process.env.LDRAW_LIB}/CAlicense4.txt`));
LDRAWRouter.use("/license/CAreadme.txt", express.static(`${process.env.LDRAW_LIB}/CAreadme.txt`));