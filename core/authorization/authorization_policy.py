"""."""
import logging

import sqlalchemy as sql
from aiohttp_security.abc import AbstractAuthorizationPolicy

from ..persistency import systemdata

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)

SCOPES_KEY = "auth_scopes"


class AuthorizationPolicy(AbstractAuthorizationPolicy):
    def __init__(self, dbengine):
        self.dbengine = dbengine

    async def authorized_userid(self, identity):
        """Return the users identity."""
        _LOGGER.debug("authorized_userid")
        async with self.dbengine.acquire() as conn:
            where = sql.and_(
                systemdata.users.c.login == identity,
                sql.not_(systemdata.users.c.disabled),
            )
            query = systemdata.users.count().where(where)
            ret = await conn.scalar(query)
            if ret:
                return identity
            else:
                return None

    async def permits(self, identity, permission, context=None):
        """Check the users policy and grant access."""
        _LOGGER.debug("permits")
        if identity is None:
            """return if identity is not found."""
            return False

        async with self.dbengine.acquire() as conn:
            where = sql.and_(
                systemdata.users.c.login == identity,
                sql.not_(systemdata.users.c.disabled),
            )
            query = systemdata.users.select().where(where)
            ret = await conn.execute(query)
            user = await ret.fetchone()
            if user is not None:
                user_id = user[0]
                is_superuser = user[3]
                if is_superuser:
                    """Grant access if user is superuser."""
                    return True

                where = systemdata.permissions.c.user_id == user_id
                query = systemdata.permissions.select().where(where)
                ret = await conn.execute(query)
                result = await ret.fetchall()
                if ret is not None:
                    for record in result:
                        if record.perm_name == permission:
                            """Grant access if user has access to resource."""
                            return True

            """Deny access."""
            return False
