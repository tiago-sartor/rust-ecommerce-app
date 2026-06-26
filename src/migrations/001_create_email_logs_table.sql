CREATE TABLE IF NOT EXISTS email_logs (
    id         BIGSERIAL    PRIMARY KEY,
    recipient  TEXT         NOT NULL,
    subject    TEXT         NOT NULL,
    status     TEXT         NOT NULL,
    html       TEXT         NOT NULL,
    response   TEXT         NOT NULL DEFAULT '',
    sent_at    TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_email_logs_recipient  ON  email_logs(recipient);
CREATE INDEX IF NOT EXISTS idx_email_logs_status     ON  email_logs(status);
CREATE INDEX IF NOT EXISTS idx_email_logs_sent_at    ON  email_logs(sent_at);
