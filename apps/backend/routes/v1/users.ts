import { Router } from "express";
import { getwebsiteDetails, postwebsiteDetails } from "../../controllers/websiteControllers";
const router = Router();

router.post("/website", postwebsiteDetails);

router.get("/", getwebsiteDetails)
// router.get("/status/:websiteId", getwebsiteDetails)


export default router