const { invoke } = window.__TAURI__.tauri;

export async function login(email, password) {
  try {
    const result = await invoke('cmd_login', { email, password });
    return result;
  } catch (e) {
    console.error(e);
    return false;
  }
}

export async function submitTicket(userEmail, message) {
  try {
    const ticketId = await invoke('cmd_submit_ticket', { userEmail, message });
    return ticketId;
  } catch (e) {
    console.error(e);
    return null;
  }
}

export async function getTickets() {
  try {
    const tickets = await invoke('cmd_get_tickets');
    return tickets;
  } catch (e) {
    console.error(e);
    return [];
  }
}

export async function changePassword(email, oldPassword, newPassword) {
  try {
    const result = await invoke('cmd_change_password', { email, oldPassword, newPassword });
    return result;
  } catch (e) {
    console.error(e);
    return false;
  }
}

// UI handlers
document.addEventListener('DOMContentLoaded', () => {
  const loginBtn = document.getElementById('login-btn');
  const ticketSection = document.getElementById('ticket-section');
  const ticketsListSection = document.getElementById('tickets-list-section');

  loginBtn.addEventListener('click', async () => {
    const email = document.getElementById('login-email').value;
    const password = document.getElementById('login-password').value;
    const success = await login(email, password);
    if (success) {
      document.getElementById('login-section').style.display = 'none';
      ticketSection.style.display = 'block';
      ticketsListSection.style.display = 'block';
    } else {
      alert('Login failed');
    }
  });

  const submitBtn = document.getElementById('submit-ticket-btn');
  submitBtn.addEventListener('click', async () => {
    const userEmail = document.getElementById('ticket-email').value;
    const message = document.getElementById('ticket-message').value;
    const ticketId = await submitTicket(userEmail, message);
    if (ticketId) {
      alert(`Ticket submitted with ID ${ticketId}`);
      document.getElementById('ticket-email').value = '';
      document.getElementById('ticket-message').value = '';
      await refreshTickets();
    } else {
      alert('Failed to submit ticket');
    }
  });

  const refreshBtn = document.getElementById('refresh-tickets-btn');
  async function refreshTickets() {
    const tickets = await getTickets();
    const list = document.getElementById('tickets-list');
    list.innerHTML = '';
    tickets.forEach(ticket => {
      const li = document.createElement('li');
      li.textContent = `${ticket.ticket_id}: ${ticket.subject} (${ticket.status})`;
      list.appendChild(li);
    });
  }
  refreshBtn.addEventListener('click', refreshTickets);
});
