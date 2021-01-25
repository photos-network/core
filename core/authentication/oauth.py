#i guess the ground work is here 


from pymongo import MongoClient
from asyncio import run as r 
from flask_oauthlib.provider import  OAuth1Provider
from flask import *
import secrets as sec

conn = MongoClient('localhost', 27017)
db = conn.get_database("client")
col = db.get_collection('oauth')
app = Flask(__name__)
oauth = OAuth1Provider(app)


class Client(object):
    async def add_redirect_uri(self, uri, scope) -> None:
        
        col.insert_one({
            "uri":uri,
            "scope":scope
        })


    async def get_default_redirect(self) -> str:
        return col.find_one({'uri':"127.0.0.1:5000/authorized"})['uri']


    async def generate_credentials(self) -> tuple:
        return sec.token_urlsafe(12), sec.token_urlsafe(32)


class Grant(object):
    async def save_grant(self, userId, userSecret, userScope) -> None:
        col.add_one({
            "userId":userId,
            "userSecret":userSecret,
            "userScope":userScope
        })
    

    async def revoke_access(self, user_id) -> str:
        col.delete_one({"userId":user_id})
        return "access was revoked"
