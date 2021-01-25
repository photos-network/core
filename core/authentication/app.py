from flask import *
#from flask_sessions import Session
from user import Main
from asyncio import run as r
import secrets

inst = Main()
app = Flask(__name__)
app.secret_key = b'somekijuhygtrflkjh secret key'

@app.route('/signup')
def fuck():
    return render_template('signup.html') 


@app.route("/api/v1/signup", methods=['POST'])
def create_user():
    r(inst.create_user(
        request.form.get("mail"),
        request.form.get('user'),
        request.form.get("psw")
    ))
    return "yaya"


@app.route('/login')
def login():
    return render_template('login.html')


@app.route("/api/v1/user/login", methods=['POST'])
def Login():
    if r(inst.checkPass(request.form.get("username"), request.form.get("password"))):
        session['user_id'] = r(inst.get_id(request.form.get('username')))
        return "success"
    flash("sorry")


@app.route("/@me")
def get_user():
    return session['user_id']


@app.route("/authorized")
if __name__ == "__main__":
 
    app.run(port=9092, debug=True)