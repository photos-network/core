import pymongo
from asyncio import run as r
from bcrypt import hashpw, checkpw, gensalt
from datetime import datetime

conn = pymongo.MongoClient("localhost", 27017)
db = conn.get_database("users")
col = db.get_collection("user")

class Main(object):    

    async def hashPass(self, passwd) -> bytes:
        return hashpw(str(passwd).encode('utf8'), gensalt())


    async def checkPass(self, user ,passwd) -> bool:
        psw = col.find_one({'username':user})['passwd']
        return checkpw(str(passwd).encode('utf8'), psw)


    async def create_user(self, email ,user, pasw):
        col.insert_one({
            "email":email,
            "username":user,
            "passwd":r(self.hashPass(pasw)),
            #this might not work as intended but if not i will fix it
            "created_at":datetime.now(),
            "last_login":datetime.now()
        }) 


inst = Main()

