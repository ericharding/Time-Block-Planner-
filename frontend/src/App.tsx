import { createSignal, onMount, Show } from 'solid-js';
import { Login } from './components/Login';
import { Planner } from './components/Planner';
import { auth } from './api/client';
import './App.css';

function App() {
  const [isAuthenticated, setIsAuthenticated] = createSignal(false);
  const [isLoading, setIsLoading] = createSignal(true);
  const [user, setUser] = createSignal<{ id: number; username: string } | null>(null);

  onMount(async () => {
    try {
      const userData = await auth.me();
      setUser(userData);
      setIsAuthenticated(true);
    } catch (err) {
      setIsAuthenticated(false);
    } finally {
      setIsLoading(false);
    }
  });

  const handleLogin = async () => {
    try {
      const userData = await auth.me();
      setUser(userData);
      setIsAuthenticated(true);
    } catch (err) {
      console.error('Failed to fetch user data', err);
    }
  };

  const handleLogout = async () => {
    try {
      await auth.logout();
      setIsAuthenticated(false);
      setUser(null);
    } catch (err) {
      console.error('Logout failed', err);
    }
  };

  return (
    <Show when={!isLoading()} fallback={<div class="loading-screen">Loading...</div>}>
      <Show when={isAuthenticated()} fallback={<Login onLogin={handleLogin} />}>
        <div class="app">
          <div class="user-bar">
            <span>Welcome, {user()?.username}!</span>
            <button onClick={handleLogout}>Logout</button>
          </div>
          <Planner />
        </div>
      </Show>
    </Show>
  );
}

export default App;
