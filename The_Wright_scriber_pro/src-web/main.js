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
    await invoke('cmd_change_password', { email, oldPassword, newPassword });
    return true;
  } catch (e) {
    console.error(e);
    return false;
  }
}

// Additional frontend logic for showing modals, handling transcription, export, logs, tickets, etc., will go here.
