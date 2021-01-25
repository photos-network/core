import pymongo
from asyncio import run as r
from bcrypt import hashpw, checkpw, gensalt
from datetime import datetime
from string import digits as d
import random


conn = pymongo.MongoClient("localhost", 27017)
db = conn.get_database("users")
col = db.get_collection("user")


class Main():
    async def hashPass(self, passwd) -> bytes:
        return hashpw(str(passwd).encode('utf8'), gensalt()) 
    
    
    async def checkPass(self, user, passwd) -> bool:
        psw = col.find_one({'username':user})['passwd']
        return checkpw(str(passwd).encode('utf8'), psw)
    

    async def create_user(self, email, user, passwd) -> None:
        col.insert_one({
            "email":email,
            "user":user,
            'passwd':r(self.hashPass(passwd)),
            'created_at':datetime.now(),
            'last_login':datetime.now(),
        }) 


    async def get_id(self, username) -> int:
        return col.find_one({'username':username}).get('_id', 1)

    
  
