import LoginForm from "../../components/login/LoginForm";

type Props = {};
function page({}: Props) {
  return (
    <main className="flex flex-1 flex-col items-center justify-between ">
      <section>
        <LoginForm />
      </section>
    </main>
  );
}
export default page;
