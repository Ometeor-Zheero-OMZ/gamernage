"use client";

import Head from "next/head";
import axios from "axios";
import styles from "@/styles/Menu.module.css";
import { Inter } from "next/font/google";
import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import { useAuth } from "../context/AuthContext";

const inter = Inter({ subsets: ["latin"] });

type Menu = {
  id: number;
  tableId: number;
  price: number;
  cookTimeSeconds: number;
  name: string;
  menu: string;
};

export default function Menu() {
  const router = useRouter();
  const { user, loading } = useAuth();
  const [menus, setMenus] = useState<Menu[]>([]);
  const [tableId, setTableId] = useState<number | null>(null);
  const [isSending, setIsSending] = useState(false);

  useEffect(() => {
    const queryParams = new URLSearchParams(window.location.search);
    const tableIdFromQuery = queryParams.get("tableId"); // Access query parameters

    if (!loading && !user) {
      router.push("/login");
    }
    if (!loading && user && tableIdFromQuery) {
      const tableIdNumber = parseInt(tableIdFromQuery, 10);
      setTableId(tableIdNumber);
      axios
        .get(`/api/menu`, {
          headers: {
            Authorization: `Bearer ${user.token}`,
          },
        })
        .then((res) => {
          console.log(res.data);
          setMenus(res.data);
        })
        .catch((error) => {
          alert(`Error fetching data: ${error}`);
        });
    }
  }, [user, loading, router]);

  const addOrder = (tableId: number, menuId: number, menuName: string) => {
    setIsSending(true);
    console.log("Menu ID:", menuId);
    axios
      .post(
        `/api/order`,
        {
          restaurant_table_id: tableId,
          menu_id: menuId,
        },
        {
          headers: {
            Authorization: `Bearer ${user?.token}`,
            "Content-Type": "application/json",
          },
        }
      )
      .then((res) => {
        console.log(res.data);
        alert(`"${menuName}" was added to the order`);
        setIsSending(false);
      })
      .catch((error) => {
        alert(`Error fetching data: ${error}`);
        setIsSending(false);
      });
  };

  const toMinutesAndSeconds = (totalSeconds: number) => {
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;
    return `${minutes} minutes ${seconds} seconds`;
  };

  return (
    <>
      <Head>
        <title>Restaurant App</title>
        <meta name="description" content="Restaurant App" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main className={`${styles.main} ${inter.className}`}>
        <h2>Table {tableId}</h2>
        <ul>
          {menus.length === 0 ? (
            <div>No Menu found</div>
          ) : (
            menus.map((menu) => (
              <li className={`${styles.menu}`} key={menu.id}>
                <ul>
                  <li>{menu.name}</li>
                  <li>
                    Expected to take {toMinutesAndSeconds(menu.cookTimeSeconds)}
                  </li>
                  <li>{menu.price} yen</li>
                  <li>
                    <button
                      className={`${styles.deleteOrderButton}`}
                      type="button"
                      onClick={() => addOrder(tableId!, menu.id, menu.name)}
                      disabled={isSending}
                    >
                      Add this to order
                    </button>
                  </li>
                </ul>
              </li>
            ))
          )}
        </ul>
      </main>
    </>
  );
}
