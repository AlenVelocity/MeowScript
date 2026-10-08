import { stdlib } from "@/generated/stdlib";

export function StdlibReference() {
  return (
    <>
      <h2 id="standard-library">Standard library</h2>
      <p>
        These tables come from the interpreter itself (<code>meowscript packages --json</code>), so they can&apos;t
        drift from the code. Arities are checked before a function runs: the wrong number of arguments is a{" "}
        <em>Paws off!</em> error, not a silent <code>mew</code>.
      </p>

      <h3 id="always-available">Always available</h3>
      <FunctionTable functions={stdlib.prelude} />

      {stdlib.packages.map((pkg) => (
        <section key={pkg.name}>
          <h3 id={pkg.name.replace(":", "-")}>
            <code>pawckage &quot;{pkg.name}&quot;;</code>
          </h3>
          <p>{pkg.doc}</p>
          <FunctionTable functions={pkg.functions} />
          {pkg.constants.length > 0 && (
            <table>
              <thead>
                <tr>
                  <th>Constant</th>
                  <th>Value</th>
                </tr>
              </thead>
              <tbody>
                {pkg.constants.map((c) => (
                  <tr key={c.name}>
                    <td>
                      <code>{c.name}</code>
                    </td>
                    <td>{String(c.value)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </section>
      ))}
    </>
  );
}

function FunctionTable({ functions }: { functions: { name: string; arity: string; doc: string }[] }) {
  return (
    <table>
      <thead>
        <tr>
          <th>Pawction</th>
          <th>Takes</th>
          <th>Does</th>
        </tr>
      </thead>
      <tbody>
        {functions.map((f) => (
          <tr key={f.name}>
            <td>
              <code>{f.name}</code>
            </td>
            <td>{f.arity}</td>
            <td>{f.doc}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}
