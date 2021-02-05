import sqlalchemy as sql

metadata = sql.MetaData()


users = sql.Table(
    "users",
    metadata,
    sql.Column("id", sql.Integer, nullable=False),
    sql.Column("login", sql.String(256), nullable=False),
    sql.Column("passwd", sql.String(256), nullable=False),
    sql.Column("is_superuser", sql.Boolean, nullable=False, server_default="FALSE"),
    sql.Column("disabled", sql.Boolean, nullable=False, server_default="FALSE"),
    # indices
    sql.PrimaryKeyConstraint("id", name="user_pkey"),
    sql.UniqueConstraint("login", name="user_login_key"),
)


permissions = sql.Table(
    "permissions",
    metadata,
    sql.Column("id", sql.Integer, nullable=False),
    sql.Column("user_id", sql.Integer, nullable=False),
    sql.Column("perm_name", sql.String(64), nullable=False),
    # indices
    sql.PrimaryKeyConstraint("id", name="permission_pkey"),
    sql.ForeignKeyConstraint(
        ["user_id"], [users.c.id], name="user_permission_fkey", ondelete="CASCADE"
    ),
)
