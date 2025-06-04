import { Router } from "express";
import WebsiteRouter from "./websites"
import UserRouter from "./users"
const router =  Router()

router.use("/users", UserRouter)
router.use("/users", WebsiteRouter)

export default router;