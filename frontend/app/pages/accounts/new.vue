<script setup>
import { invoke } from "@tauri-apps/api/core";

const formData = ref({
  username: "",
  incoming: {
    protocol: "imap",
    server: "",
    port: 993,
    address: "",
    password: "",
  },
  outgoing: {
    protocol: "smtp",
    server: "",
    port: 587,
    address: "",
    password: "",
  },
});

const createAccount = () => {
  const v = formData.value;
  invoke("accounts_add", {
    username: v.username,
    incoming: {
      IMAP: {
        server: v.incoming.server,
        port: v.incoming.port,
        address: v.incoming.address,
      },
    },
    outgoing: {
      SMTP: {
        server: v.outgoing.server,
        address: v.outgoing.address,
      },
    },
    imap_password: v.incoming.password,
    smtp_password: v.outgoing.password,
  })
    .then(async (_) => await navigateTo("/accounts/list"))
    .catch((err) => console.error(err));
};
</script>

<template>
  <Centerer>
    <div class="account-form">
      <h1>New account</h1>

      <div class="form-field">
        <label for="username">Username</label>
        <input
          id="username"
          v-model="formData.username"
          type="text"
          placeholder="Enter username"
        />
      </div>

      <div class="connections-container">
        <div class="connection-section">
          <h3>Incoming</h3>
          <div class="form-field">
            <label for="incoming-protocol">Protocol</label>
            <select id="incoming-protocol" v-model="formData.incoming.protocol">
              <option value="imap">IMAP</option>
            </select>
          </div>

          <div
            v-if="formData.incoming.protocol === 'imap'"
            class="protocol-fields"
          >
            <div class="form-field">
              <label for="imap-server">Server</label>
              <input
                id="imap-server"
                v-model="formData.incoming.server"
                type="text"
                placeholder="imap.example.com"
              />
            </div>
            <div class="form-field">
              <label for="imap-port">Port</label>
              <input
                id="imap-port"
                v-model="formData.incoming.port"
                type="number"
                placeholder="993"
              />
            </div>
            <div class="form-field">
              <label for="imap-address">Email Address</label>
              <input
                id="imap-address"
                v-model="formData.incoming.address"
                type="email"
                placeholder="user@example.com"
              />
            </div>
            <div class="form-field">
              <label for="imap-password">Password</label>
              <input
                id="imap-password"
                v-model="formData.incoming.password"
                type="password"
                placeholder="Password"
              />
            </div>
          </div>
        </div>

        <div class="connection-section">
          <h3>Outgoing</h3>
          <div class="form-field">
            <label for="outgoing-protocol">Protocol</label>
            <select id="outgoing-protocol" v-model="formData.outgoing.protocol">
              <option value="smtp">SMTP</option>
            </select>
          </div>

          <div
            v-if="formData.outgoing.protocol === 'smtp'"
            class="protocol-fields"
          >
            <div class="form-field">
              <label for="smtp-server">Server</label>
              <input
                id="smtp-server"
                v-model="formData.outgoing.server"
                type="text"
                placeholder="smtp.example.com"
              />
            </div>
            <!-- <div class="form-field">
              <label for="smtp-port">Port</label>
              <input
                id="smtp-port"
                v-model="formData.outgoing.port"
                type="number"
                placeholder="587"
              />
            </div> -->
            <div class="form-field">
              <label for="smtp-address">Email Address</label>
              <input
                id="smtp-address"
                v-model="formData.outgoing.address"
                type="email"
                placeholder="user@example.com"
              />
            </div>
            <div class="form-field">
              <label for="smtp-password">Password</label>
              <input
                id="smtp-password"
                v-model="formData.outgoing.password"
                type="password"
                placeholder="Password"
              />
            </div>
          </div>
        </div>
      </div>

      <div class="form-actions">
        <button @click="createAccount" class="save-btn">Create Account</button>
      </div>
    </div>
  </Centerer>
</template>

<style scoped>
.account-form {
  width: 50%;
  padding: 16px;
  background-color: #1e1e2e;
  border-radius: 8px;
  border: 1px solid #313244;
}

.form-field {
  margin-bottom: 4px;
}

.form-field label {
  display: block;
  margin-bottom: 4px;
  font-weight: 500;
  margin-left: 8px;
}

.form-field input,
.form-field select {
  border: 1px solid #313244;
  border-radius: 4px;
  font-size: 14px;
}

.form-field input {
  padding: 8px;
  width: calc(100% - 32px);
}

.form-field select {
  width: calc(100% - 16px);
  height: 36px;
}

.form-field input:focus,
.form-field select:focus {
  outline: none;
  border-color: #89b4fa;
}

.connections-container {
  display: flex;
  gap: 24px;
  margin-top: 24px;
}

.connection-section {
  flex: 1;
  background-color: #11111b;
  padding: 16px;
  border-radius: 6px;
  border: 1px solid #313244;
}

.connection-section h3 {
  margin-bottom: 16px;
  color: #89b4fa;
}

.protocol-fields {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.form-actions {
  margin-top: 24px;
  text-align: right;
}

.save-btn {
  background-color: #89b4fa;
  color: #11111b;
  border: none;
  padding: 10px 20px;
  border-radius: 4px;
  font-weight: 500;
  cursor: pointer;
}

.save-btn:hover {
  background-color: #74c7ec;
}
</style>
