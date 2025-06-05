import { Router } from "express";
import { getwebsiteDetails, postwebsiteDetails } from "../../controllers/websiteControllers";
const router = Router();

router.post("/", postwebsiteDetails);

router.get("/status/:websiteId", getwebsiteDetails)


export default router