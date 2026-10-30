import { LocalIcon } from "@krizaka/icons";
import { ProductLogo } from "@krizaka/ui";
import { AppShell } from "@krizaka/ui/app-shell";
import { Button } from "@krizaka/ui/button";
import { useCallback, useEffect, useState } from "react";

import { PRODUCT } from "./config";
import { bridge, folders, followLinks, setNativeMenu } from "./desktop";
import { useI18n } from "./i18n";
import { menuSpec } from "./menu";

/**
 * The window (krizaka-ui ADR-002): `AppShell desktop` — the sidebar and the header are the title bar, the traffic lights are
 * cleared, desktop density. The first screen lists the folders the person granted; replace it with the product's own
 * screens, built from @krizaka/ui blocks (and the AccountMenu in the header once the product signs in).
 */
export function App() {
  const { t, messages } = useI18n();
  const [granted, setGranted] = useState<string[]>([]);
  const refresh = useCallback(() => void folders.list().then(setGranted), []);
  const add = useCallback(() => void folders.grant().then(refresh), [refresh]);

  useEffect(refresh, [refresh]);
  // The native menu in the app's words, and the deep links (<scheme>://app/… becomes the window's hash).
  useEffect(() => {
    const stops = [
      setNativeMenu(bridge, menuSpec(messages.menu), { "folders.add": add }),
      followLinks(bridge, (route) => (window.location.hash = route)),
    ];
    return () => void Promise.all(stops).then((all) => all.forEach((stop) => stop()));
  }, [messages, add]);

  return (
    <AppShell.Root desktop>
      <AppShell.Sidebar
        label={t("app.name")}
        brand={
          <>
            <ProductLogo id={PRODUCT as Parameters<typeof ProductLogo>[0]["id"]} size={22} animated={false} />{" "}
            {t("app.name")}
          </>
        }
        collapseLabels={[t("shell.collapse"), t("shell.expand")]}
        closeLabel={t("shell.close")}
      >
        <AppShell.Item href="#/" icon={<LocalIcon size={16} />} label={t("folders.title")} current />
      </AppShell.Sidebar>
      <AppShell.Body>
        <AppShell.Header
          title={t("folders.title")}
          menuLabel={t("shell.menu")}
          actions={
            <Button size="sm" variant="primary" onClick={add}>
              {t("folders.add")}
            </Button>
          }
        />
        <AppShell.Main className="space-y-4">
          <p className="text-fg-secondary">{t("app.tagline")}</p>
          {granted.length === 0 ? <p className="text-fg-secondary">{t("folders.empty")}</p> : null}
          <ul className="flex flex-col divide-y divide-border-subtle">
            {granted.map((folder) => (
              <li key={folder} className="flex items-center justify-between gap-2 py-2">
                <span className="truncate">{folder}</span>
                <Button size="sm" variant="ghost" onClick={() => void folders.revoke(folder).then(setGranted)}>
                  {t("folders.remove")}
                </Button>
              </li>
            ))}
          </ul>
        </AppShell.Main>
      </AppShell.Body>
    </AppShell.Root>
  );
}
