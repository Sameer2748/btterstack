import { Router } from "express";
import { getwebsiteDetails, postwebsiteDetails } from "../../controllers/websiteControllers";
const router = Router();

router.post("/", (req, res, next) => {
  postwebsiteDetails(req, res).catch(next);
});

router.get("/status/:websiteId", (req, res, next) => {
  getwebsiteDetails(req, res).catch(next);
});

export default router