import { api } from 'boot/axios'
import { Notify } from 'quasar'


export async function validateToken() {

  const tkn = localStorage.getItem('auth_tkn');
  if (!tkn) {
    return false;
  }

  const result = await api.get('/api/idm/sso', {headers: {'Authorization': `Bearer ${tkn}`}})
    .catch((err) => {
      Notify.create({
        message: err.message,
        color: 'negative',
        position: 'top'
      });
        return { status: 401 };
      });

  if (result.status === 401) {
    localStorage.removeItem('auth_tkn');
    return false;
  }

  return true;

}
