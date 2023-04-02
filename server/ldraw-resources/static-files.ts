import express, { Router } from "express";
import { BundleRouter } from "./bundle/routes";
import dotenv from "dotenv";

dotenv.config();

export const LDRAWRouter = Router();

LDRAWRouter.use("/zip", BundleRouter)

LDRAWRouter.use("/data/parts", express.static(`${process.env.LDRAW_LIB}/parts`));
LDRAWRouter.use("/data/parts", express.static(`${process.env.LDRAW_LIB}/p`));

LDRAWRouter.use("/config/LDConfig.ldr", express.static(`${process.env.LDRAW_LIB}/LDConfig.ldr`));