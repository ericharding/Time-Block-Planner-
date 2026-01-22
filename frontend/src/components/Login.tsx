import { createSignal } from 'solid-js';
import { auth } from '../api/client';
import './Login.css';

interface LoginProps {
  onLogin: () => void;
}

export function Login(props: LoginProps) {
  const [username, setUsername] = createSignal('');
  const [password, setPassword] = createSignal('');
  const [isRegistering, setIsRegistering] = createSignal(false);
  const [error, setError] = createSignal('');
  const [loading, setLoading] = createSignal(false);

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    setError('');
    setLoading(true);

    try {
      if (isRegistering()) {
        await auth.register(username(), password());
      } else {
        await auth.login(username(), password());
      }
      props.onLogin();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Authentication failed');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div class="login-container">
      <div class="login-card">
        <h1>📅 Daily Time Planner</h1>
        <h2>{isRegistering() ? 'Create Account' : 'Sign In'}</h2>

        <form onSubmit={handleSubmit}>
          <div class="form-group">
            <label>Username</label>
            <input
              type="text"
              value={username()}
              onInput={(e) => setUsername(e.currentTarget.value)}
              required
              disabled={loading()}
            />
          </div>

          <div class="form-group">
            <label>Password</label>
            <input
              type="password"
              value={password()}
              onInput={(e) => setPassword(e.currentTarget.value)}
              required
              minLength={6}
              disabled={loading()}
            />
          </div>

          {error() && <div class="error">{error()}</div>}

          <button type="submit" disabled={loading()}>
            {loading()
              ? 'Please wait...'
              : isRegistering()
              ? 'Create Account'
              : 'Sign In'}
          </button>
        </form>

        <div class="toggle">
          {isRegistering() ? 'Already have an account?' : "Don't have an account?"}
          <button
            type="button"
            class="link-btn"
            onClick={() => setIsRegistering(!isRegistering())}
          >
            {isRegistering() ? 'Sign In' : 'Create Account'}
          </button>
        </div>
      </div>
    </div>
  );
}
